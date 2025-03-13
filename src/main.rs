pub mod commands;
pub mod sshcfg;

use blkc::*;
use commands::*;
use std::process::exit;

#[allow(dead_code)]
struct Command {
    name:           &'static str,
    description:    &'static str,
    option:         &'static str,
    run:            fn(&'static str, &'static str, &'static Vec<String>, &'static Vec<Server>)
}

static DESCRIPTIONS: [&str; 7] = [
    "Show server list.",
    "Run a command on a single remote host.",
    "Run a command on a multiple remote host.",
    "Run a command as root on a single remote host.",
    "Run a command as root on a multiple remote host.",
    "Print help menu.",
    "Show version."
];

static COMMANDS: [Command; 14] = [
    Command{ name: "--show",    description: DESCRIPTIONS[0], option: "",           run: print_server_details       },
    Command{ name: "-s",        description: DESCRIPTIONS[0], option: "",           run: print_server_details       },
    Command{ name: "--run",     description: DESCRIPTIONS[1], option: "--name",     run: single_remote_command      },
    Command{ name: "--run",     description: DESCRIPTIONS[2], option: "--label",    run: multi_remote_command       },
    Command{ name: "-r",        description: DESCRIPTIONS[1], option: "-n",         run: single_remote_command      },
    Command{ name: "-r",        description: DESCRIPTIONS[2], option: "-l",         run: multi_remote_command       },
    Command{ name: "--srun",    description: DESCRIPTIONS[3], option: "--name",     run: single_root_remote_command },
    Command{ name: "--srun",    description: DESCRIPTIONS[4], option: "--label",    run: multi_root_remote_command  },
    Command{ name: "-x",        description: DESCRIPTIONS[3], option: "-n",         run: single_root_remote_command },
    Command{ name: "-x",        description: DESCRIPTIONS[4], option: "-l",         run: multi_root_remote_command  },
    Command{ name: "--help",    description: DESCRIPTIONS[5], option: "",           run: help                       },
    Command{ name: "-h",        description: DESCRIPTIONS[5], option: "",           run: help                       },
    Command{ name: "--version", description: DESCRIPTIONS[6], option: "",           run: version                    },
    Command{ name: "-v",        description: DESCRIPTIONS[6], option: "",           run: version                    },
];

fn main() {
    let args            : Vec<String>           = std::env::args().collect();
    let colors          : Vec<String>           = get_colors(&args);
    let static_args     : &'static Vec<String>  = Box::leak(Box::new(args));
    let static_colors   : &'static Vec<String>  = Box::leak(Box::new(colors));

    let servers_json = match server_list() {
        Ok(json) => json,
        Err(err) => { eprintln!("Error: {err}"); exit(1) }
    };
    let servers: Vec<Server> = serde_json::from_str(servers_json).expect("Failed to deserialize.");
    let static_servers: &'static Vec<Server> = Box::leak(Box::new(servers));

    let command = if static_args.len() > 1 { static_args[1].trim() } else { "" };
    let opt     = if static_args.len() > 2 { static_args[2].trim() } else { "" };
    let query   = if static_args.len() > 3 { static_args[3].trim() } else { "" };
    let rm_cmd  = if static_args.len() > 4 { static_args[4].trim() } else { "" };

    match static_args.len() >= 5 {
        true => match COMMANDS.iter().find(|cmd| cmd.name == command && cmd.option == opt) {
            Some(cmd) => (cmd.run)(&query, rm_cmd, static_colors, static_servers),
            None => { eprintln!("Error: Command not found."); exit(1) }
        },
        false => match COMMANDS.iter().find(|cmd| cmd.name == command) {
            Some(cmd) => (cmd.run)(&opt, "", static_colors, &static_servers),
            None => { eprintln!("Error: Command not found."); exit(1) }
        },
    }
}
