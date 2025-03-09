use std::fs::File;
use std::process::Command;
use std::io::Read;
use serde::{Deserialize, Serialize};

pub const ROOT_COLOR_PREFIX: &str = "\x1b[33m";
pub const MAIN_COLOR_PREFIX: &str = "\x1b[32m";
pub const MAIN_COLOR_SUFFIX: &str = "\x1b[0m";

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Server {
    pub id: i32,
    pub label: &'static str,
    pub name: &'static str,
    pub user: &'static str,
    pub address: &'static str,
    pub sshport: &'static str
}

pub fn print_server_details(vec_data: Vec<Server>, server_name: &'static str) {
    for server in vec_data {
        match !server_name.is_empty() {
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

pub fn get_colors(args: Vec<String>) -> Vec<String> {
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

pub fn get_passkey(server_name: String) -> std::io::Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(String::from(format!("pass {}-key", server_name)))
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get password. {}", output.status),
        ))
    }
}

pub fn server_list<'a>() -> std::io::Result<&'a str> {
    let list_path = std::env::var("HOME").unwrap().to_string() + "/.config/blkc/list.json";
    let mut file = File::open(String::from(list_path)).expect("Failed to open file.");
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    let static_str: &'static str = Box::leak(json_string.into_boxed_str());
    Ok(static_str.try_into().expect("try failed"))
}

