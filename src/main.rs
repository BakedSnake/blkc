pub mod commands;

use blkc::*;
use commands::*;
#[allow(unused_imports)]
use std::thread; // TODO

#[allow(dead_code)]
struct Command {
    name: &'static str,
    description: &'static str,
    run: fn(cmd: &str, opt: &str) -> fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>)
}

static COMMANDS: [Command; 8] = [
    Command{ name: "--show", description: "Show server list.", run: command_handler },
    Command{ name: "-s", description: "Show server list.", run: command_handler },
    Command{ name: "--run", description: "Run command on a remote host, or multiple remote hosts", run: command_handler },
    Command{ name: "-r", description: "Run command on a remote host, or multiple remote hosts", run: command_handler },
    Command{ name: "--srun", description: "Run command as root on a remote host, or multiple remote hosts", run: command_handler },
    Command{ name: "-x", description: "Run command as root on a remote host, or multiple remote hosts", run: command_handler },
    Command{ name: "--help", description: "Print help menu.", run: command_handler },
    Command{ name: "-h", description: "Print help menu.", run: command_handler },
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let static_args: &'static Vec<String> = Box::leak(Box::new(args.clone()));
    let colors: Vec<String> = get_colors(args.clone());
    let static_colors: &'static Vec<String> = Box::leak(Box::new(colors.clone()));

    if args.len() < 1 {
        return
    }

    let servers_json = match server_list() {
        Ok(json) => json,
        Err(err) => { eprintln!("Error: {err}"); return }
    };
    let servers: Vec<Server> = serde_json::from_str(servers_json).expect("Failed to deserialize.");
    let static_servers: &'static Vec<Server> = Box::leak(Box::new(servers.clone()));

    let command = if static_args.len() > 1 { static_args[1].trim() } else { "" };
    let opt     = if static_args.len() > 2 { static_args[2].trim() } else { "" };
    let query   = if static_args.len() > 3 { static_args[3].trim() } else { "" };
    let rm_cmd  = if static_args.len() > 4 { static_args[4].trim() } else { "" };

    match static_args.len() {
        5 => match COMMANDS.iter().find(|cmd| cmd.name == command) {
            Some(cmd) => (cmd.run)(command, opt)(&query, rm_cmd, static_colors, static_servers),
            None => {
                eprintln!("Error: Unknown command.");
                std::process::exit(1)
            }
        },
        3 => match COMMANDS.iter().find(|cmd| cmd.name == command) {
            Some(cmd) => {
                if opt != "all" {
                    (cmd.run)(command, opt) (&opt, "", static_colors, &static_servers);
                } else {
                    (cmd.run)(command, opt) ("", "", static_colors, &static_servers);
                }
            }
            None => {
                eprintln!("Error: Unknown command.");
                std::process::exit(1)
            }
        },
        1 => match COMMANDS.iter().find(|cmd| cmd.name == command) {
            Some(cmd) => (cmd.run)(command, opt)("", "", static_colors, &static_servers),
            None => {
                eprintln!("Error: Unknown command.");
                std::process::exit(1)
            }
        },
        _ => match COMMANDS.iter().find(|cmd| cmd.name == "--help") {
            Some(cmd) => (cmd.run)(command, opt)("", "", static_colors, &static_servers),
            None => {
                eprintln!("Error: Unknown command.");
                std::process::exit(1)
            }
        },
    }
}

fn command_handler(cmd: &str, opt: &str) -> fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>) {
    match cmd {
        "-r" => match opt {
            "--name" => single_remote_command,
            "-n" => single_remote_command,
            "--label" => multi_remote_command,
            "-l" => multi_remote_command,
            _ => panic!("Not a real command option.")
        },
        "--run" => match opt {
            "--name" => single_remote_command,
            "-n" => single_remote_command,
            "--label" => multi_remote_command,
            "-l" => multi_remote_command,
            _ => panic!("Not a real command option.")
        },
        "-x" => match opt {
            "--name" => single_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            "-n" => single_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            "--label" => multi_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            "-l" => multi_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            _ => panic!("Not a real command option.")
        },
        "--srun" => match opt {
            "--name" => single_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            "-n" => single_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            "--label" => multi_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            "-l" => multi_root_remote_command
                as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
            _ => panic!("Not a real command option.")
        },
        "-s" => print_server_details,
        "--show" => print_server_details,
        "-h" => help
            as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
        "--help" => help
            as fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>),
        _ => panic!("Invalid command.")
    }
}
