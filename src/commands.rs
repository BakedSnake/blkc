use ssh::*;
use blkc::*;
use std::io::Read;
use std::str::from_utf8;

pub fn remote_commands(server_label: &str, command: &str, colors: Vec<String>) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");

    for server in &vec_data {
        match server.label == server_label {
            true => {
                let mut session = Session::new().unwrap();
                session.set_host(&server.name.to_lowercase()).unwrap();
                session.parse_config(None).unwrap();
                session.connect().unwrap();
                println!("{:?}",session.is_server_known());
                let pass_key = user_pass(server.name.to_string()).unwrap();
                session.userauth_publickey_auto(Some(&pass_key)).unwrap();

                let cmd = command.as_bytes();
                let mut channel = session.channel_new().unwrap();
                channel.open_session().unwrap();
                channel.request_exec(cmd).unwrap();
                channel.send_eof().unwrap();
                let mut buf = Vec::new();
                channel.stdout().read_to_end(&mut buf).unwrap();
                let output = from_utf8(&buf).unwrap();

                println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server.name,  colors[1], colors[2], command);
                println!("{}-------------------------{}", colors[1], colors[2]);
                print!("{output}");

            },
            false => eprintln!("No server found...")
        }

    }

}

pub fn remote_command(server_name: &str, command: &str, colors: Vec<String>) {
    let mut session = Session::new().unwrap();
    session.set_host(&server_name.to_lowercase()).unwrap();
    session.parse_config(None).unwrap();
    session.connect().unwrap();
    println!("{:?}",session.is_server_known());
    let pass_key = user_pass(server_name.to_string()).unwrap();
    session.userauth_publickey_auto(Some(&pass_key)).unwrap();

    let cmd = command.as_bytes();
    let mut channel = session.channel_new().unwrap();
    channel.open_session().unwrap();
    channel.request_exec(cmd).unwrap();
    channel.send_eof().unwrap();
    let mut buf = Vec::new();
    channel.stdout().read_to_end(&mut buf).unwrap();
    let output = from_utf8(&buf).unwrap();

    println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server_name,  colors[1], colors[2], command);
    println!("{}-------------------------{}", colors[1], colors[2]);
    print!("{output}");
}

