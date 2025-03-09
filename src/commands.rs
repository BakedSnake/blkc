use ssh::*;
use blkc::*;
use std::io::Read;
use std::str::from_utf8;
use std::{thread, usize};

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
        match handle.join() {
            Ok(h) => h,
            Err(err) => { eprintln!("Error: {err:?}"); return }
        }
    }
}

pub fn single_remote_command(server_name: &str, command: &str, colors: Vec<String>) {
    let session = get_session(server_name);
    run_command(session, server_name, command, colors);
}

fn run_command(mut session: Session, server_name: &str, command: &str, colors: Vec<String>) {
    let cmd = command.as_bytes();
    let mut channel = match session.channel_new() {
        Ok(chan) => chan,
        Err(err) => { eprintln!("Error: {err}"); return }
    };

    match channel.open_session() {
        Ok(chan) => chan,
        Err(err) => eprintln!("Error: {err}")
    }

    match channel.request_exec(cmd) {
        Ok(chan) => chan,
        Err(err) => eprintln!("Error: {err}")
    }

    match channel.send_eof() {
        Ok(chan) => chan,
        Err(err) => eprintln!("Error: {err}")
    }

    let mut buf = Vec::new();
    match channel.stdout().read_to_end(&mut buf) {
        Ok(chan) => chan,
        Err(_) => 0 as usize
    };

    let output = match from_utf8(&buf) {
        Ok(out) => out,
        Err(err) => { eprintln!("Error: {err}"); return }

    };

    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server_name,  colors[1], colors[2], command);
    println!("{}-------------------------{}", colors[1], colors[2]);
    print!("{output}\n");
}

fn get_session(server_name: &str) -> Session {
    let mut session = Session::new().unwrap();
    session.set_host(&server_name.to_lowercase()).unwrap();

    match session.parse_config(None) {
        Ok(config) => config,
        Err(err) => eprintln!("Error: {err}")
    }

    match session.connect() {
        Ok(conn) => conn,
        Err(err) => eprintln!("Error: {err}")
    }

    //println!("{:?}",session.is_server_known());
    let pass_key = match get_passkey(server_name.to_string()) {
        Ok(pk) => pk,
        Err(_) => String::new()
    };

    match session.userauth_publickey_auto(Some(&pass_key)) {
        Ok(pk) => pk,
        Err(err) => eprintln!("Error: {err}")
    }

    session
}
