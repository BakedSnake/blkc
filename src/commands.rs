use ssh2::Session;
use blkc::*;
use crate::sshcfg::get_session;
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

pub fn single_root_remote_command(server_name: &str, command: &str, colors: &'static Vec<String>, _: &'static Vec<Server>) {
    let session = get_session(server_name);
    run_root_command(session, server_name, command, colors.to_vec());
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

pub fn single_remote_command(server_name: &'static str, command: &'static str, colors: &'static Vec<String>, _: &'static Vec<Server>) {
    let session = get_session(server_name);
    run_command(session, server_name, command, colors.to_vec());
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

pub fn print_server_details(server_name: &'static str, _command: &'static str, _: &'static Vec<String>, servers: &'static Vec<Server>) {
    for server in servers {
        match server_name != "all" {
            true => match server.name == server_name {
                true => print!(
                    "Name: {}\nUser: {}\nAddress: {}\nSSH Port: {}\nLabel: {}\n--------------------\n",
                    server.name, server.user, server.address, server.sshport, server.label
                ),
                false => ()
            },
            false => match server.id > 0 {
                true => print!(
                    "Name: {}\nUser: {}\nAddress: {}\nSSH Port: {}\nLabel: {}\n--------------------\n",
                    server.name, server.user, server.address, server.sshport, server.label
                ),
                false => ()
            }
        }
    }
}

pub fn help(_: &str, _: &str, colors: &Vec<String>, _: &Vec<Server>) {
    println!("{}Usage:", colors[1]);
    println!("-------------------------{}", colors[2]);
    println!("blkc [--run|srun] [--name|label] name|label [command [argument...]]\n");
    println!("--nocolor,    -C    Disable color output");
    println!("--run,        -r    Run command as user");
    println!("--srun,       -x    Run command as root user");
    println!("--name,       -n    Name of the server");
    println!("--label,      -l    Label of the server\n");
    println!("`--srun` and `--run` cannot be used at the same time.\nThe same goes for `--name` and `--label`.\n")
}
