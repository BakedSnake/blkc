use blkc::*;
use crate::COMMANDS;
use crate::sshcfg::get_session;
use std::thread;

pub fn root_remote_command(query: &str, command: &'static str, opt: &'static str, colors: &'static Vec<String>, servers: &'static Vec<Server>) {
    if opt == "-n" || opt == "--name" {
        for server in servers {
            match server.name == query {
                true => {
                    let session = get_session(server.name);
                    run_root_command(session, server.name, command, colors.to_vec());
                },
                false => continue
            }
        }
    }

    if opt == "-l" || opt == "--label" {
        let mut handles = Vec::new();

        for server in servers {
            match server.label == query {
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
}

pub fn remote_command(query: &'static str, command: &'static str, opt: &'static str, colors: &'static Vec<String>, servers: &'static Vec<Server>) {
    if opt == "-n" || opt == "--name" {
        for server in servers {
            match server.name == query {
                true => {
                    let session = get_session(server.name);
                    run_command(session, server.name, command, colors.to_vec());
                },
                false => continue
            }
        }
    }

    if opt == "-l" || opt == "--label" {
        let mut handles = Vec::new();

        for server in servers {
            match server.label == query {
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
}

pub fn print_server_details(server_name: &'static str, _command: &'static str, opt: &'static str, colors: &'static Vec<String>, servers: &'static Vec<Server>) {
    for server in servers {
        let print_d = format!(
            "{}Name:{} {}\n{}User:{} {}\n{}Address:{} {}\n{}SSH Port:{} {}\n{}Label:{} {}\n--------------------\n",
            colors[1], colors[2], server.name, colors[1], colors[2], server.user, colors[1], colors[2], server.address,
            colors[1], colors[2], server.sshport, colors[1], colors[2], server.label
        );

        if opt == "-n" || opt == "--name" || opt == "-l" || opt == "--label"{
            match server_name != "all" {
                true => match server.name == server_name {
                    true => print!("{print_d}"),
                    false => match server.label == server_name {
                        true => print!("{print_d}"),
                        false => ()
                    }
                },
                false => match server.id > 0 {
                    true => print!("{print_d}"),
                    false => ()
                }
            }
        }
    }
}

pub fn help(_: &str, _: &str, _opt: &str, colors: &Vec<String>, _: &Vec<Server>) {
    println!("{}Usage:", colors[1]);
    println!("-------------------------{}", colors[2]);
    println!("{}blkc [{}--run|srun{}] [{}--name|label{}]{} name|label {}[{}command {}[{}argument...{}]]{}\n",
        colors[0], colors[2], colors[0], colors[2], colors[0], colors[2], colors[0], colors[2], colors[0], colors[2], colors[0], colors[2]
    );

    for command in COMMANDS.iter() {
        if !command.option.is_empty() {
            println!("{}{} {} [ target ]:{}\n \t{}\n", colors[0], command.name, command.option, colors[2], command.description)
        } else {
            println!("{}{}:{} {}\n \t{}\n", colors[0], command.name, command.option, colors[2], command.description)
        }
    }
    println!("{}-C:{}\tDisable color output\n", colors[0], colors[2]);
    println!("`--srun` and `--run` cannot be used at the same time.\nThe same goes for `--name` and `--label`.\n")
}

pub fn version(_: &str, _: &str, _: &str, colors: &Vec<String>, _: &Vec<Server>) {
    println!("{}blkc:{} v0.2.1", colors[1], colors[2]);
}
