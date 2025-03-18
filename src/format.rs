use blkc::*;
use crate::COMMANDS;

pub const ROOT_COLOR_PREFIX: &str = "\x1b[33m";
pub const MAIN_COLOR_PREFIX: &str = "\x1b[32m";
pub const MAIN_COLOR_SUFFIX: &str = "\x1b[0m";

pub fn print_root_result(server_name: &str, command: &str, buffer: String) {
    let colors = get_colors();
    let root_color_prefix = &colors[0];
    let main_color_suffix = &colors[2];

    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", root_color_prefix, main_color_suffix, server_name,  root_color_prefix, main_color_suffix, command);
    println!("{}-------------------------{}", root_color_prefix, main_color_suffix);
    print!("{buffer}\n");
}

pub fn print_result(server_name: &str, command: &str, buffer: String) {
    let colors = get_colors();
    let main_color_prefix = &colors[1];
    let main_color_suffix = &colors[2];

    println!();
    println!("{} Label: {} {} -> {} Command: {} {}", main_color_prefix, main_color_suffix, server_name,  main_color_prefix, main_color_suffix, command);
    println!("{}-------------------------{}", main_color_prefix, main_color_suffix);
    print!("{buffer}\n");
}

pub fn print_details_result(server: &Server) {
    let colors = get_colors();
    let main_color_prefix = &colors[1];
    let main_color_suffix = &colors[2];

    print!("{}Name:{} {}\n{}User:{} {}\n{}Address:{} {}\n{}SSH Port:{} {}\n{}Label:{} {}\n--------------------\n",
        main_color_prefix, main_color_suffix, server.name, main_color_prefix, main_color_suffix, server.user, main_color_prefix, main_color_suffix, server.address,
        main_color_prefix, main_color_suffix, server.sshport, main_color_prefix, main_color_suffix, server.label
    );
}

pub fn print_help_command() {
    let colors = get_colors();
    let root_color_prefix = &colors[0];
    let main_color_prefix = &colors[1];
    let main_color_suffix = &colors[2];

    println!("{}Usage:", main_color_prefix);
    println!("-------------------------{}", main_color_suffix);
    println!("{}blkc [{}--run|srun{}] [{}--name|label{}]{} name|label {}[{}command {}[{}argument...{}]]{}\n",
        root_color_prefix, main_color_suffix, root_color_prefix, main_color_suffix, root_color_prefix, main_color_suffix,
        root_color_prefix, main_color_suffix, root_color_prefix, main_color_suffix, root_color_prefix, main_color_suffix
    );

    for command in COMMANDS.iter() {
        if !command.option.is_empty() {
            println!("{}{} {} [ target ]:{}\n \t{}\n", root_color_prefix, command.name, command.option, main_color_suffix, command.description)
        } else {
            println!("{}{}:{} {}\n \t{}\n", root_color_prefix, command.name, command.option, main_color_suffix, command.description)
        }
    }

    println!("{}-C:{}\tDisable color output\n", root_color_prefix, main_color_suffix);
    println!("`--srun` and `--run` cannot be used at the same time.\nThe same goes for `--name` and `--label`.\n")
}

pub fn print_version() {
    let colors = get_colors();
    let main_color_prefix = &colors[1];
    let main_color_suffix = &colors[2];

    println!("{}blkc:{} v0.2.2", main_color_prefix, main_color_suffix);
}

pub fn get_colors() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    let empty = String::from("");
    let colors: Vec<String>;

    if args.contains(&"-C".to_string()) || args.contains(&"--nocolor".to_string()) {
        let em = &empty.clone();
        colors = vec![em.to_string(), em.to_string(), em.to_string()];
    } else {
        colors = vec![
            String::from(ROOT_COLOR_PREFIX),
            String::from(MAIN_COLOR_PREFIX),
            String::from(MAIN_COLOR_SUFFIX)
        ]
    }
    colors
}

