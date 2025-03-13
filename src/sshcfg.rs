use ssh2::Session;
use blkc::{server_list,Server,get_sshkey};
use std::net::TcpStream;
use std::path::Path;

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
