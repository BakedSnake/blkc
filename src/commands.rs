use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;
use blkc::*;
use std::io::{Read, Write};
use std::thread;

pub fn multi_root_remote_command(server_label: &str, command: &'static str, colors: &'static Vec<String>, servers: &'static Vec<Server>) {
    let mut handles = Vec::new();

    for server in servers {
        match server.label == server_label {
            true => {
                let handle = thread::spawn(move || {
                    let session = get_session(server.name);
                    run_root_command(session, server.name, command, colors.to_vec());
                });
                handles.push(handle);
            },
            false => continue
        }
    }

    for handle in handles {
        match handle.join() {
            Ok(h) => h,
            Err(err) => { eprintln!("Error: {err:?}"); return }
        }
    }
}

pub fn single_root_remote_command(server_name: &str, command: &str, colors: Vec<String>) {
    let session = get_session(server_name);
    run_root_command(session, server_name, command, colors);
}

pub fn multi_remote_command(server_label: &'static str, command: &'static str, colors: &'static Vec<String>, servers: &'static Vec<Server>) {
    let mut handles = Vec::new();

    for server in servers {
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

pub fn run_root_command(session: Session, server_name: &str, command: &str, colors: Vec<String>) {
    let password = get_userpass(server_name.to_string()).unwrap();
    let pass_fmt = format!("{password}\n");
    let cmd = format!("sudo {command}");
    let mut channel = session.channel_session().unwrap();

    channel.request_pty("vt10", None, None).unwrap();
    channel.exec(&cmd).unwrap();
    channel.write_all(pass_fmt.as_bytes()).unwrap();
    channel.send_eof().unwrap();
    let mut buf = String::new();
    channel.read_to_string(&mut buf).unwrap();

    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", colors[0], colors[2], server_name,  colors[0], colors[2], command);
    println!("{}-------------------------{}", colors[0], colors[2]);
    print!("{buf}\n");

    channel.wait_close().unwrap();
}

fn run_command(session: Session, server_name: &str, command: &str, colors: Vec<String>) {
    let mut channel = session.channel_session().unwrap();

    channel.request_pty("vt10", None, None).unwrap();
    channel.exec(&command).unwrap();
    channel.send_eof().unwrap();
    let mut buf = String::new();
    channel.read_to_string(&mut buf).unwrap();

    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server_name,  colors[1], colors[2], command);
    println!("{}-------------------------{}", colors[1], colors[2]);
    print!("{buf}\n");

    channel.wait_close().unwrap();
}

pub fn get_session(server_name: &str) -> Session {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize...");
    let (mut server_sshport,mut server_user,mut server_address) = ("", "", "");
    let key_path = get_sshkey();
    for server in &vec_data {
        if server.name == server_name {
            server_sshport = server.sshport;
            server_user = server.user;
            server_address = server.address;
        }
    }

    let tcp = TcpStream::connect(format!("{}:{}", server_address, server_sshport)).unwrap();
    let mut sess = Session::new().unwrap();
    let _agent = sess.agent().unwrap();
    let pubkey_str = format!("{}.pub", key_path);
    let pubkey_path = Path::new(&pubkey_str);
    let privkey_path = Path::new(&key_path);

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_pubkey_file(server_user, Some(pubkey_path), privkey_path, Some("")).unwrap();
    assert!(sess.authenticated());

    sess

}

