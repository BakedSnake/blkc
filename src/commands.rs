use ssh::*;
use blkc::*;
use std::io::Read;
use std::str::from_utf8;
use std::thread;

pub fn remote_commands(server_label: &'static str, command: &'static str, colors: &'static Vec<String>) {
    let mut handles = Vec::new();
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");

    for server in vec_data {
        match server.label == server_label {
            true => {
                let handle = thread::spawn(move || {
                    let mut session = Session::new().unwrap();
                    session.set_host(&server.name.to_lowercase()).unwrap();
                    session.parse_config(None).unwrap();

                    session.connect().unwrap();
                    //println!("{:?}",session.is_server_known());
                    let pass_key = get_passkey(server.name.to_string()).unwrap();
                    session.userauth_publickey_auto(Some(&pass_key)).unwrap();

                    let cmd = command.as_bytes();
                    let mut channel = session.channel_new().unwrap();
                    channel.open_session().unwrap();
                    channel.request_exec(cmd).unwrap();
                    channel.send_eof().unwrap();
                    let mut buf = Vec::new();
                    channel.stdout().read_to_end(&mut buf).unwrap();
                    let output = from_utf8(&buf).unwrap();

                    println!();
                    println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server.name,  colors[1], colors[2], command);
                    println!("{}-------------------------{}", colors[1], colors[2]);
                    print!("{output}\n");

                });
                handles.push(handle);
            },
            false => continue
        }
    };

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn remote_command(server_name: &str, command: &str, colors: Vec<String>) {
    let mut session = Session::new().unwrap();
    session.set_host(&server_name.to_lowercase()).unwrap();
    session.parse_config(None).unwrap();
    session.connect().unwrap();
    //println!("{:?}",session.is_server_known());
    let pass_key = get_passkey(server_name.to_string()).unwrap();
    session.userauth_publickey_auto(Some(&pass_key)).unwrap();

    let cmd = command.as_bytes();
    let mut channel = session.channel_new().unwrap();
    channel.open_session().unwrap();
    channel.request_exec(cmd).unwrap();
    channel.send_eof().unwrap();
    let mut buf = Vec::new();
    channel.stdout().read_to_end(&mut buf).unwrap();
    let output = from_utf8(&buf).unwrap();

    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server_name,  colors[1], colors[2], command);
    println!("{}-------------------------{}", colors[1], colors[2]);
    print!("{output}\n");
}

