use ssh2::Session;
use std::path::Path;
use std::net::TcpStream;
use std::io::{Read, Write};
use blkc::{server_list,
    Server,
    get_sshkey,
    get_userpass,
};
use crate::format::{
    print_result,
    print_root_result
};

pub fn run_root_command(session: Session, server_name: &str, command: &str) {
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

    print_root_result(server_name, command, buf);

    channel.wait_close().unwrap();
}

pub fn run_command(session: Session, server_name: &str, command: &str) {
    let mut channel = session.channel_session().unwrap();

    channel.request_pty("vt10", None, None).unwrap();
    channel.exec(&command).unwrap();
    channel.send_eof().unwrap();
    let mut buf = String::new();
    channel.read_to_string(&mut buf).unwrap();

    print_result(server_name, command, buf);

    channel.wait_close().unwrap();
}

pub fn get_session(server_name: &str) -> Session {
    let server = get_server_to_conn(server_name);
    let tcp = get_tcp_conn(&server);
    let mut sess = Session::new().unwrap();

    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let _ = ssh_auth(&sess, server.user);
    assert!(sess.authenticated());

    sess
}

pub fn ssh_auth(sess: &Session, server_user: &str) -> std::io::Result<()> {
    let _agent = sess.agent().unwrap();
    let key_path = match get_sshkey() {
        Ok(path) => path,
        Err(err) => return Err(err)
    };
    let pubkey_str = format!("{}.pub", key_path);
    let pubkey_path = Path::new(&pubkey_str);
    let privkey_path = Path::new(&key_path);

    match sess.userauth_pubkey_file(server_user, Some(pubkey_path), privkey_path, Some("")) {
        Ok(_) => (),
        Err(err) => eprintln!("Error: {err}")
    }

    Ok(())
}

pub fn get_tcp_conn(server: &Server) -> TcpStream {
    let tcp_stream = match TcpStream::connect(format!("{}:{}", server.address, server.sshport)) {
        Ok(tcp) => tcp,
        Err(err) => panic!("Error: {err}")
    };

    tcp_stream
}

pub fn get_server_to_conn(server_name: &str) -> Server {
    let mut server_to_conn = Server::new();
    let servers_json = match server_list() {
        Ok(json) => json,
        Err(err) => { eprintln!("Error: {err}"); "" }
    };
    let servers: Vec<Server> = serde_json::from_str(servers_json).expect("Failed to deserialize.");

    for server in &servers {
        if server.name == server_name {
            server_to_conn = server.clone();
        }
    }

    server_to_conn
}
