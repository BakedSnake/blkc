pub mod commands;

use blkc::*;
use commands::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let static_args: &'static Vec<String> = Box::leak(Box::new(args.clone()));
    let colors = get_colors(args.clone());
    let servers_json = server_list().unwrap();
    let servers = serde_json::from_str(servers_json).expect("Failed to deserialize.");

    match static_args.len() {
        5 => {
            match args[1].contains("--run") {
                true => {
                    match &args[2].contains("--name") {
                        true => remote_command(&args[3], &args[4], colors.clone()),
                        false => ()
                    }
                    match &args[2].contains("--label") {
                        true => remote_commands(&args[3], &args[4], colors.clone()),
                        false => ()
                    }
                },
                false => return
            }
        },
        3 => {
            match static_args[1].contains("--show") || static_args[1].contains("-s") {
                true => match static_args[2].contains("all") {
                    true => print_server_details(servers, ""),
                    false => print_server_details(servers, &static_args[2])
                },
                false => return
            }
        },
        2 => {
            match static_args[1].contains("--help") || static_args[1].contains("-h") {
                true => help(colors.clone()),
                false => return
            }
        },
        _ => help(colors)

    }
}

fn help(colors: Vec<String>) {
    println!("{}Usage:", colors[1]);
    println!("-------------------------{}", colors[2]);
    println!("blkc [--run|srun] [--name|label] name|label [command [argument...]]\n");
    println!("--nocolor,    -C    Disable color output");
    println!("--run,        -r    Run command as user");
    println!("--srun,       -sr   Run command as root user");
    println!("--name,       -n    Name of the server");
    println!("--label,      -l    Label of the server\n");
    println!("`--srun` and `--run` cannot be used at the same time.\nThe same goes for `--name` and `--label`.\n")
}
