use std::fs::File;
use std::process::Command;
use std::io::{BufRead, BufReader, Read};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Server {
    pub id: i32,
    pub label: &'static str,
    pub name: &'static str,
    pub user: &'static str,
    pub address: &'static str,
    pub sshport: &'static str
}

impl Server {
    pub fn new() -> Self {
        Self { id: -2, label: "None", name: "None", user: "None", address: "None", sshport: "None" }
    }
}

pub fn parse_cmd_name<'a>(cmd_name: &'a str, cmd: &'a str) -> &'a str {
    let names: Vec<&str> = cmd_name.split("| ").collect();
    let long = names[0];
    let short = names[1];

    if cmd == long { return long }
    if cmd == short { return short } else { "" }
}

pub fn get_userpass(server_name: String) -> std::io::Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(String::from(format!("pass {}", server_name)))
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get password. {}", output.status)))
    }
}

pub fn get_sshkey() -> std::io::Result<String> {
    let home = match std::env::var("HOME") {
        Ok(env) => env,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    };
    let config = "/.config/blkc/blkc.conf";
    let config_path = String::from(home + config);
    let file = match File::open(config_path) {
        Ok(cfg) => cfg,
        Err(err) => return Err(err)
    };
    let reader = BufReader::new(file);
    let mut key_path = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            if key == "ssh_key" {
                key_path = value.to_string();
            }
        }
    }

    Ok(key_path)
}

pub fn server_list<'a>() -> std::io::Result<&'a str> {
    let home = match std::env::var("HOME") {
        Ok(env) => env,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
    };
    let config = "/.config/blkc/list.json";
    let list_path = String::from(home + config);
    let mut file = match File::open(list_path) {
        Ok(list) => list,
        Err(err) => return Err(err)
    };

    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    let static_str: &'static str = Box::leak(json_string.into_boxed_str());
    Ok(static_str.try_into().expect("try failed"))
}

