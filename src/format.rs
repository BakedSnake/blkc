use blkc::*;
use crate::COMMANDS;

pub fn print_root_result(server_name: &str, command: &str, buffer: String) {
    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX, server_name,  ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX, command);
    println!("{}-------------------------{}", ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX);
    print!("{buffer}\n");
}

pub fn print_result(server_name: &str, command: &str, buffer: String) {
    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX, server_name,  MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX, command);
    println!("{}-------------------------{}", MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX);
    print!("{buffer}\n");
}

pub fn print_details_result(server: &Server) {
    print!("{}Name:{} {}\n{}User:{} {}\n{}Address:{} {}\n{}SSH Port:{} {}\n{}Label:{} {}\n--------------------\n",
        MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX, server.name, MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX, server.user, MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX, server.address,
        MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX, server.sshport, MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX, server.label
    );
}

pub fn print_help_command() {
    println!("{}Usage:", MAIN_COLOR_PREFIX);
    println!("-------------------------{}", MAIN_COLOR_SUFFIX);
    println!("{}blkc [{}--run|srun{}] [{}--name|label{}]{} name|label {}[{}command {}[{}argument...{}]]{}\n",
        ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX, ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX, ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX,
        ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX, ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX, ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX
    );

    for command in COMMANDS.iter() {
        if !command.option.is_empty() {
            println!("{}{} {} [ target ]:{}\n \t{}\n", ROOT_COLOR_PREFIX, command.name, command.option, MAIN_COLOR_SUFFIX, command.description)
        } else {
            println!("{}{}:{} {}\n \t{}\n", ROOT_COLOR_PREFIX, command.name, command.option, MAIN_COLOR_SUFFIX, command.description)
        }
    }

    println!("{}-C:{}\tDisable color output\n", ROOT_COLOR_PREFIX, MAIN_COLOR_SUFFIX);
    println!("`--srun` and `--run` cannot be used at the same time.\nThe same goes for `--name` and `--label`.\n")
}

pub fn print_version() {
    println!("{}blkc:{} v0.2.1", MAIN_COLOR_PREFIX, MAIN_COLOR_SUFFIX);
}
