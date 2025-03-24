use blkc::*;
use std::thread;
use crate::format::*;
use crate::sshcfg::{
    get_session,
    run_command,
    run_root_command,
};

pub fn multi_root_remote_command(query: &str, command: &'static str, servers: &'static Vec<Server>) {
    let mut handles = Vec::new();

    for server in servers {
        match server.label == query {
            true => {
                let handle = thread::spawn(move || {
                    let session = get_session(server.name);
                    run_root_command(session, server.name, command);
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

pub fn root_remote_command(query: &str, command: &'static str, servers: &'static Vec<Server>) {
    for server in servers {
        match server.name == query {
            true => {
                let session = get_session(server.name);
                run_root_command(session, server.name, command);
            },
            false => continue
        }
    }
}

pub fn multi_remote_command(query: &'static str, command: &'static str, servers: &'static Vec<Server>) {
    let mut handles = Vec::new();

    for server in servers {
        match server.label == query {
            true => {
                let handle = thread::spawn(move || {
                    let session = get_session(server.name);
                    run_command(session, server.name, command);

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

pub fn remote_command(query: &'static str, command: &'static str, servers: &'static Vec<Server>) {
    for server in servers {
        match server.name == query {
            true => {
                let session = get_session(server.name);
                run_command(session, server.name, command);
            },
            false => continue
        }
    }
}

pub fn print_server_details(server_name: &'static str, servers: &'static Vec<Server>) {
    for server in servers {
        match server_name != "all" {
            true => match server.name == server_name {
                true => print_details_result(&server),
                false => match server.label == server_name {
                    true => print_details_result(&server),
                    false => ()
                }
            },
            false => match server.id > 0 {
                true => print_details_result(&server),
                false => ()
            }
        }
    }
}
