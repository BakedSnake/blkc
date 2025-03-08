use blkc::*;
use std::process::Stdio;
use futures::stream::{FuturesUnordered, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let static_args: &'static Vec<String> = Box::leak(Box::new(args.clone()));
    let colors = get_colors(args);

    if static_args.len() > 2 {
        if static_args[1].contains("--show") || static_args[1].contains("-s") {
            if static_args[2].contains("all") {
                print_server_details(serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize."), "");
            } else {
                print_server_details(serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize."), &static_args[2]);
            }
        }
        if static_args[1].contains("--run") || static_args[1].contains("-r") {
            if static_args[2].contains("--label") || static_args[2].contains("-l") {
                futures::executor::block_on(run_multi_command(&static_args[3], &static_args[4], &colors));
            } else if static_args[2].contains("--name") || static_args[2].contains("-n"){
                futures::executor::block_on(run_command(&static_args[3], &static_args[4], &colors));
            } else {
                eprintln!("Wrong input")
            }
        } 
        if static_args[1].contains("--srun") || static_args[1].contains("-sr") {
            if static_args[2].contains("--label") || static_args[2].contains("-l") {
                futures::executor::block_on(run_multi_command_as_root(&static_args[3], &static_args[4], &colors));
            } else if static_args[2].contains("--name") || static_args[2].contains("-n"){
                futures::executor::block_on(run_command_as_root(&static_args[3], &static_args[4], &colors));
            } else {
                eprintln!("Wrong input")
            }
        }
        if static_args[1].contains("--help") || static_args[1].contains("-h") {
            help(colors);
        }
    } else {
        if static_args.len() <= 1 {
            help(colors);
        } else {
            help(colors);
        }
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

async fn run_multi_command_as_root(server_label: &str, command: &'static str, colors: &Vec<String>) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    let mut tasks = Vec::new();
    let mut futures = FuturesUnordered::new();
    for server in &vec_data {
        if server.label == server_label {
            let server_name = server.name;
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let handle = async move {
                match root_cmd(server_name, server_sshport, server_user, server_address, command).await {
                    Ok(out) => { 
                        println!("\n{}ROOT{} {}Label: {} {} -> {} Command: {} {}", colors[0], colors[2], colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("\n{}\n", out)
                    },
                    Err(err) => {

                        println!("\n{}ROOT{} {}Label: {} {} -> {} Command: {} {}", colors[0], colors[2], colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("{}", err)
                    }
                };
            };
            tasks.push(Box::pin(handle));
        }
    }
    futures.extend(tasks);
    let _ = futures.collect::<Vec<_>>().await;
}

async fn run_multi_command(server_label: &str, command: &'static str, colors: &Vec<String>) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    let mut tasks = Vec::new();
    let mut futures = FuturesUnordered::new();
    for server in &vec_data {
        if server.label == server_label {
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let task = async move {
                match cmd(server_sshport, server_user, server_address, command).await {
                    Ok(out) => { 
                        println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("{}\n", out) 
                    },
                    Err(err) => {
                        println!("{} Label: {} {} -> {} Command: {} {}", colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("{}", err)
                    }
                };
            };
            tasks.push(Box::pin(task));
        }
    }
    futures.extend(tasks);
    let _ = futures.collect::<Vec<_>>().await;
}

async fn run_command_as_root(server_name: &str, command: &'static str, colors: &Vec<String>) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    let mut tasks = Vec::new();
    let mut futures = FuturesUnordered::new();
    for server in &vec_data {
        if server.name == server_name {
            let server_name = server.name;
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let handle = async move {
                match root_cmd(server_name, server_sshport, server_user, server_address, command).await {
                    Ok(out) => {
                        println!("\n{}ROOT{} {}Server: {} {} -> {} Command: {} {}", colors[0], colors[2], colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("\n{}\n", out)
                    },
                    Err(err) => {
                        println!("\n{}ROOT{} {}Server: {} {} -> {} Command: {} {}", colors[0], colors[2], colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("{}", err)
                    }
                };
            };
            tasks.push(Box::pin(handle));
        }
    }
    futures.extend(tasks);
    let _ = futures.collect::<Vec<_>>().await;
}

async fn run_command(server_name: &str, command: &'static str, colors: &Vec<String>) {
    let vec_data: Vec<Server> = serde_json::from_str(&server_list().unwrap()).expect("Failed to deserialize.");
    let mut tasks = Vec::new();
    let mut futures = FuturesUnordered::new();
    for server in &vec_data {
        if server.name == server_name {
            let server_sshport = server.sshport;
            let server_user = server.user;
            let server_address = server.address;
            let handle = async move {
                match cmd(server_sshport, server_user, server_address, command).await {
                    Ok(out) => { 
                        println!("{} Server: {} {} -> {} Command: {} {}", colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("{}\n", out)
                    },
                    Err(err) => {
                        println!("{} Server: {} {} -> {} Command: {} {}", colors[1], colors[2], server.name,  colors[1], colors[2], command);
                        println!("{}-------------------------{}", colors[1], colors[2]);
                        println!("{}", err)
                    }
                };
            };
            tasks.push(Box::pin(handle));
        }
    }
    futures.extend(tasks);
    let _ = futures.collect::<Vec<_>>().await;
}

async fn root_cmd(server_name: &str, port: &str, user: &str, address: &str, command: &str) -> std::io::Result<String> {
    let mut output = tokio::process::Command::new("ssh")
          .args(&["-i",  &config_ssh().unwrap(), "-t",  "-p",  port, &format!("{}@{}", user, address), "sudo", "-S", command])
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .stderr(Stdio::null())
          .spawn()?;
    if let Some(mut stdin) = output.stdin.take() {
        stdin.write_all(user_pass(server_name.to_string())?.as_bytes()).await?;
    } else {
        return Err(std::io::Error::new(
                std::io::ErrorKind::Other, 
                "Failed to open stdin"
                ));
    }
    let child_stdout: Option<tokio::process::ChildStdout> = output.stdout.take();
    let result = async_output_result(child_stdout).await;
    let output = output.wait_with_output().await?;
    if output.status.success() {
        Ok(result.trim().to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command '{}' failed with exit code {}", command, output.status),
        ))
    }
}

async fn cmd(port: &str, user: &str, address: &str, command: &str) -> std::io::Result<String> {
    let mut output = tokio::process::Command::new("ssh")
          .args(&["-i",  &config_ssh().unwrap(), "-t",  "-p",  port, &format!("{}@{}", user, address), command ])
          .stdin(Stdio::piped())
          .stdout(Stdio::piped())
          .stderr(Stdio::null())
          .spawn()?;
    let child_stdout: Option<tokio::process::ChildStdout> = output.stdout.take();
    let result = async_output_result(child_stdout).await;
    let output = output.wait_with_output().await?;
    if output.status.success() {
        Ok(result)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command '{}' failed with exit code {}", command, output.status),
        ))
    }
}

async fn async_output_result(stdout: Option<tokio::process::ChildStdout>) -> String {
    if let Some(mut stdout) = stdout {
        let mut outres_str = String::new();
        stdout.read_to_string(&mut outres_str).await.expect("Hit me daddy!");
        outres_str
    } else {
        String::new()
    }
}
