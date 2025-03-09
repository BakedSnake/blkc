use ssh::*;
use blkc::*;
use std::io::Read;
use std::str::from_utf8;
use std::thread;

pub fn multi_remote_command(server_label: &'static str, command: &'static str, colors: &'static Vec<String>) {
    let mut handles = Vec::new();
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");

    for server in vec_data {
        match server.label == server_label {
            true => {
                let handle = thread::spawn(move || {
                    let session = get_session(server.name);
                    run_command(session, server.name, command, colors.to_vec());

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

pub fn single_remote_command(server_name: &str, command: &str, colors: Vec<String>) {
    let session = get_session(server_name);
    run_command(session, server_name, command, colors);
}

fn run_command(mut session: Session, server_name: &str, command: &str, colors: Vec<String>) {
    let cmd = command.as_bytes();
    let channel = &mut session.channel_new().unwrap();
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

fn get_session(server_name: &str) -> Session {
    let mut session = Session::new().unwrap();
    session.set_host(&server_name.to_lowercase()).unwrap();
    session.parse_config(None).unwrap();
    session.connect().unwrap();
    //println!("{:?}",session.is_server_known());
    let pass_key = get_passkey(server_name.to_string()).unwrap();
    session.userauth_publickey_auto(Some(&pass_key)).unwrap();

    session
}
