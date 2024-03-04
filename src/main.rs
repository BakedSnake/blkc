use std::fs::File;
use std::io::{Read, Write, BufRead};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::{thread, usize};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let static_args: &'static Vec<String> = Box::leak(Box::new(args.clone()));
    if static_args.len() > 2 {
        if static_args[1].contains("--show") || static_args[1].contains("-s") {
            if static_args[2].contains("all") {
                print_server_details(serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize."), "");
            } else {
                print_server_details(serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize."), &static_args[2]);
            }
        } else if static_args[1].contains("--run") || static_args[1].contains("-r") {
            if static_args[2].contains("--label") || static_args[2].contains("-l") {
                run_multi_command(&static_args[3], &static_args[4]);
            } else if static_args[2].contains("--name") || static_args[2].contains("-n"){
                run_command(&static_args[3], &static_args[4]);
            } else {
                eprintln!("Wrong input")
            }
        } 
        if static_args[1].contains("--srun") || static_args[1].contains("-sr") {
            if static_args[2].contains("--label") || static_args[2].contains("-l") {
                run_multi_command_as_root(&static_args[3], &static_args[4]);
            } else if static_args[2].contains("--name") || static_args[2].contains("-n"){
                run_command_as_root(&static_args[3], &static_args[4]);
            } else {
                eprintln!("Wrong input")
            }
        }
        if static_args[1].contains("--help") || static_args[1].contains("-h") {
            help();
        }
    } else {
        help();
    }

}

fn help() {
    println!("{}Usage:", "\x1b[32m");
    println!("-------------------------{}", "\x1b[0m");
    println!("blkc --[run|srun] --[name|label] name|label command\n");
    println!("--run,   -r    Run command as user");
    println!("--srun,  -sr   Run command as root user");
    println!("--name,  -n    Name of the server");
    println!("--label, -n    Label of the server\n");
    println!("--srun and --run cannot be used at the same time.\nThe same goes for --name and --label.\n")
}

fn run_multi_command_as_root(server_label: &str, command: &'static str) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    for server in &vec_data {
        if server.label == server_label {
            println!("{}ROOT{} {}Label: {} {} -> {} Command: {} {}", "\x1b[33m", "\x1b[0m", "\x1b[32m", "\x1b[0m", server.name,  "\x1b[32m", "\x1b[0m", command);
            println!("{}-------------------------{}", "\x1b[32m", "\x1b[0m");
            let server_name = server.name;
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let handle = thread::spawn(move || {
                match root_cmd(server_name, server_sshport, server_user, server_address, command) {
                    Ok(out) => println!("\n{}\n", out),
                    Err(err) => println!("{}", err)
                };
            });
            handle.join().unwrap();
        }
    }
    
}

fn run_multi_command(server_label: &str, command: &'static str) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    for server in &vec_data {
        if server.label == server_label {
            println!("{} Label: {} {} -> {} Command: {} {}", "\x1b[32m", "\x1b[0m", server.name,  "\x1b[32m", "\x1b[0m", command);
            println!("{}-------------------------{}", "\x1b[32m", "\x1b[0m");
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let handle = thread::spawn(move || {
                match cmd(server_sshport, server_user, server_address, command) {
                    Ok(out) => println!("{}\n", out),
                    Err(err) => println!("{}", err)
                };
            });
            handle.join().unwrap();
        }
    }
    
}

fn run_command_as_root(server_name: &str, command: &'static str) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    for server in &vec_data {
        if server.name == server_name {
            println!("{}ROOT{} {}Server: {} {} -> {} Command: {} {}", "\x1b[33m", "\x1b[0m", "\x1b[32m", "\x1b[0m", server.name,  "\x1b[32m", "\x1b[0m", command);
            println!("{}-------------------------{}", "\x1b[32m", "\x1b[0m");
            let server_name = server.name;
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let handle = thread::spawn(move || {
                match root_cmd(server_name, server_sshport, server_user, server_address, command) {
                    Ok(out) => println!("\n{}\n", out),
                    Err(err) => println!("{}", err)
                };
            });
            handle.join().unwrap();
        }
    }
}

fn run_command(server_name: &str, command: &'static str) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    for server in &vec_data {
        if server.name == server_name {
            println!("{} Server: {} {} -> {} Command: {} {}", "\x1b[32m", "\x1b[0m", server.name,  "\x1b[32m", "\x1b[0m", command);
            println!("{}-------------------------{}", "\x1b[32m", "\x1b[0m");
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let handle = thread::spawn(move || {
                match cmd(server_sshport, server_user, server_address, command) {
                    Ok(out) => println!("{}\n", out),
                    Err(err) => println!("{}", err)
                };
            });
            handle.join().unwrap();
        }
    }
}

fn root_cmd(server_name: &str, port: &str, user: &str, address: &str, command: &str) -> std::io::Result<String> {
    let output = Command::new("ssh")
          .args(&["-i",  &config_ssh().unwrap(), "-t",  "-p",  port, &format!("{}@{}", user, address), "sudo", "-S", command])
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn()?;
    let mut stdin = output.stdin.as_ref().expect("Failed to open stdin");
    let _ = stdin.write_all(user_pass(server_name.to_string())?.as_bytes());
    let output = output.wait_with_output()?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command '{}' failed with exit code {}", command, output.status),
        ))
    }
}

fn cmd(port: &str, user: &str, address: &str, command: &str) -> std::io::Result<String> {
    let mut output = Command::new("ssh")
          .args(&["-i",  &config_ssh().unwrap(), "-t",  "-p",  port, &format!("{}@{}", user, address), command])
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .spawn()?;
    let stdout = output.stdout.take();
    let output_result = if let Some(mut stdout) = stdout {
        let mut outres_str = String::new();
        stdout.read_to_string(&mut outres_str).expect("Hit me daddy!"); 
        outres_str
    } else {
        String::new()
    };
    let output = output.wait_with_output()?;
    if output.status.success() {
        Ok(output_result)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command '{}' failed with exit code {}", command, output.status),
        ))
    }
}

fn print_server_details(vec_data: Vec<Server>, server_name: &'static str) {
    for server in vec_data {
        if !server_name.is_empty() {
            if server.name == server_name {
                print!(
                    "Name: {}\nUser: {}\nAddress: {}\nSSH Port: {}\nLabel: {}\n--------------------\n",
                    server.name, server.user, server.address, server.sshport, server.label
                );
            }
        } else {
            if server.id > 0 {
                print!(
                    "Name: {}\nUser: {}\nAddress: {}\nSSH Port: {}\nLabel: {}\n--------------------\n",
                    server.name, server.user, server.address, server.sshport, server.label
                );
            }
        }
    }
}

fn user_pass(server_name: String) -> std::io::Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(String::from(format!("pass {}", server_name)))
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

fn config_ssh() -> std::io::Result<String>{
    let home_cfg = std::env::var("HOME").unwrap().to_string() + "/.config/blkc/blkc.conf";
    let path_user = Path::new(&home_cfg);
    let path;
    if path_user.is_file() {
        path = path_user;
    } else {
        println!("No config file fond at ~/.config/blkc.conf");
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Error...",
        ))
    }
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let key = parts[0].trim();
            let value = parts[1].trim();
            if key == "ssh_key" {
                return Ok(value.to_string());
            }
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Error...",
    ))

}

fn server_list<'a>() -> std::io::Result<&'a str> {
    let list_path = std::env::var("HOME").unwrap().to_string() + "/.config/blkc/list.json";
    let mut file = File::open(String::from(list_path)).expect("Failed to open file.");
    let mut json_string = String::new();
    file.read_to_string(&mut json_string)?;
    let static_str: &'static str = Box::leak(json_string.into_boxed_str());
    Ok(static_str.try_into().expect("try failed"))
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Server { 
    id: i32,
    label: &'static str,
    name: &'static str,
    user: &'static str,
    address: &'static str,
    sshport: &'static str 
}
