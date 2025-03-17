pub mod commands;
pub mod sshcfg;
pub mod format;

use blkc::*;
use commands::*;
use std::process::exit;

struct Command {
    name:           &'static str,
    description:    &'static str,
    option:         &'static str,
    run:            fn(&'static str, &'static str, &'static str, &'static Vec<Server>)
}

fn main() {
    let args            : Vec<String>           = std::env::args().collect();
    let static_args     : &'static Vec<String>  = Box::leak(Box::new(args));

    let servers_json = match server_list() {
        Ok(json) => json,
        Err(err) => { eprintln!("Error: {err}"); exit(1) }
    };
    let servers: Vec<Server> = serde_json::from_str(servers_json).expect("Failed to deserialize.");
    let static_servers: &'static Vec<Server> = Box::leak(Box::new(servers));

    let command = match static_args.iter().find(|arg| VALID_COMMANDS.contains(&arg.trim())) {
        Some(arg) => arg,
        None => { println!("Command error: Command not found."); exit(1) }
    };

    let opt = match static_args.iter().find(|arg| VALID_OPTIONS.contains(&arg.trim())) {
        Some(arg) => arg,
        None => ""
    };

    let query = match static_args.windows(2).find(|pair| &pair[0].trim() == &opt) {
        Some(pair) => &pair[1],
        None => ""
    };

    let rm_cmd = match static_args.windows(2).find(|pair| &pair[0].trim() == &query) {
        Some(pair) => &pair[1],
        None => ""
    };

    match COMMANDS.iter().find(|cmd| parse_cmd_name(cmd.name.trim(), command) == command) {
        Some(cmd) => (cmd.run)(&query, rm_cmd, &opt, static_servers),
        None => { eprintln!("Error: Command not found."); exit(1) }
    }
}

static COMMANDS: [Command; 5] = [
    Command{ name: "--show| -s",    description: DESCRIPTIONS[0], option: "[-n| -l| --name| --label]",  run: print_server_details       },
    Command{ name: "--run| -r",     description: DESCRIPTIONS[1], option: "[-n| -l| --name| --label]",  run: remote_command             },
    Command{ name: "--srun| -x",    description: DESCRIPTIONS[2], option: "[-n| -l| --name| --label]",  run: root_remote_command        },
    Command{ name: "--help| -h",    description: DESCRIPTIONS[3], option: "",                           run: help                       },
    Command{ name: "--version| -v", description: DESCRIPTIONS[4], option: "",                           run: version                    },
];

static DESCRIPTIONS: [&str; 5] = [
    "Show server list.",
    "Run a command on a single or multiple remote hosts.",
    "Run a command as root on a single or multiple remote host.",
    "Print help menu.",
    "Show version."
];

static VALID_OPTIONS: [&str; 4] = [
    "--name",
    "--label",
    "-n",
    "-l"
];

static VALID_COMMANDS: [&str; 10] = [
    "--run",
    "--srun",
    "--show",
    "--help",
    "--version",
    "-r",
    "-x",
    "-s",
    "-h",
    "-v"
];

