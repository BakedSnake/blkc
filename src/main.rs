pub mod commands;
pub mod sshcfg;
pub mod format;

use blkc::*;
use commands::*;
use clap::{Parser, Subcommand};
use std::process::exit;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Cmd
}

#[derive(Subcommand, Debug)]
enum Cmd {
    #[command(name = "run", alias = "-r")]
    /// Run a command on a single or multiple remote hosts
    Run {
        /// Name of the remote host
        #[arg(short, long)]
        name: Option<String>,

        /// Label of the remote host
        #[arg(short, long)]
        label: Option<String>,

        /// Command to run on remote host
        #[arg(short, long, required = true)]
        command: String,
    },
    #[command(name = "exec", alias = "-x")]
    /// Run a command as root on a single or multiple remote hosts
    Exec {
        /// Name of the remote host
        #[arg(short, long)]
        name: Option<String>,

        /// Label of the remote host
        #[arg(short, long)]
        label: Option<String>,

        /// Command to run on remote host
        #[arg(short, long, required = true)]
        command: String,
    },
    #[command(name = "show", alias = "-s")]
    #[group(required = true, multiple = false, args = ["name", "label"])]
    /// Show server list
    Show {
        /// Name of the remote host
        #[arg(short, long)]
        name: Option<String>,

        /// Label of the remote host
        #[arg(short, long)]
        label: Option<String>,
    }
}

fn main() {
    let cli = Cli::parse();
    let static_cli = Box::leak(Box::new(cli));

    let servers_json = match server_list() {
        Ok(json) => json,
        Err(err) => { eprintln!("Error: {err}"); exit(1) }
    };
    let servers: Vec<Server> = serde_json::from_str(servers_json).expect("Failed to deserialize.");
    let static_servers: &'static Vec<Server> = Box::leak(Box::new(servers));

    match &static_cli.command {
        Cmd::Run { name, label, command } => {
            match name {
                Some(name) => if !name.is_empty() {
                    remote_command(&name, &command, &static_servers);
                },
                None => ()
            }
            match label {
                Some(label) => if !label.is_empty() {
                    multi_remote_command(&label, &command, &static_servers);
                },
                None => ()
            }
        },
        Cmd::Exec { name, label, command } => {
            match name {
                Some(name) => if !name.is_empty() {
                    root_remote_command(&name, &command, &static_servers);
                },
                None => ()
            }
            match label {
                Some(label) => if !label.is_empty() {
                    multi_root_remote_command(&label, &command, &static_servers);
                },
                None => ()
            }
        },
        Cmd::Show { name, label } => {
            match name {
                Some(name) => if !name.is_empty() {
                    print_server_details(&name, &static_servers);
                },
                None => ()
            }
            match label {
                Some(label) => if !label.is_empty() {
                    print_server_details(&label, &static_servers);
                },
                None => ()
            }
        },
    }
}
