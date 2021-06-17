use std::io::{stdout, Write};
use std::process::{exit, Stdio};

fn run(cmd: &str, args: &[&str], message: &str) {
    let child = std::process::Command::new(cmd)
        .args(args)
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect(message);
    stdout().write_all(&child.stdout).unwrap();
}

fn generate_message(initial_msg: &str, package_vector: &Vec<&str>) -> String {
    let mut message = initial_msg.to_string();
    for i in package_vector {
        message.push_str(i);
        message.push_str(", ");
    }
    message
}

fn match_op(operation: &str, cmd: &str, package_vector: &Vec<&str>) {
    match operation {
        "install" => {
            let mut args: Vec<&str> = match cmd {
                "apk" => vec!("add"),
                "apt" => vec!("install"),
                "emerge" => vec!("-a", "-t", "-v"),
                "xbps-install" => vec!("-S"),
                _ => vec!("N/A"),
            };
            for i in package_vector {
                args.push(i);
            }
            if args[0] == "N/A" {
                println!("You found a bug in the install operation of `pkg::match_op()`!");
                exit(1);
            }
            let message = generate_message("Could not install ", package_vector);
            run(cmd, &args, &message);
        },
        "remove" => {
            let cmd = if cmd == "xbps-install" {
                "xbps-remove"
            } else {
                cmd
            };
            let mut args: Vec<&str> = match cmd {
                "apk" => vec!("del"),
                "apt" => vec!("autoremove"),
                "emerge" => vec!("-a", "-v", "-c"),
                "xbps-remove" => vec!("-R"),
                _ => vec!("N/A"),
            };
            for i in package_vector {
                args.push(i);
            }
            if args[0] == "N/A" {
                println!("You found a bug in the remove operation of `pkg::match_op()`!");
                exit(1);
            }
            let message = generate_message("Could not remove ", package_vector);
            run(cmd, &args, &message);
        },
        _ => {
            println!("Sorry, {} is not a supported operation right now.", operation);
            exit(1);
        }
    }
}

fn backend(package_manager: &str, package_vector: &Vec<&str>, operation: &str) {
    match package_manager {
        "apk" => match_op(operation, "apk", package_vector),
        "apt" => match_op(operation, "apt", package_vector),
        "portage" => match_op(operation, "emerge", package_vector),
        "xbps" => match_op(operation, "xbps-install", package_vector),
        _ => {
            println!("Sorry, {} is not a supported package manager right now.", package_manager);
            exit(1);
        }
    }
}

pub fn manage(package_manager: &str, packages: &str, operation: &str) {
    let packages = packages.replace("\n", " ");
    let package_vector = packages.split(" ").collect::<Vec<&str>>();
    backend(&package_manager, &package_vector, &operation);
}
