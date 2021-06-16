use std::io::{stdout, Write};
use std::process::{exit, Stdio};

fn run(cmd: &str, args: &[&str], message: &str) {
    let child = std::process::Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect(message);
    stdout().write_all(&child.stdout).unwrap();
}

fn match_op(operation: &str, cmd: &str, package: &str) {
    match operation {
        "install" => {
            let args: Vec<&str> = match cmd {
                "emerge" => vec!("-a", "-t", "-v", &package),
                "xbps-install" => vec!("-S", &package),
                _ => vec!("N/A"),
            };
            if args[0] == "N/A" {
                println!("You found a bug, it shouldn't be possible to reach this but I had to cover this to make rust happy.");
                exit(1);
            }
            let message = ["Could not install ", &package].concat();
            run(cmd, &args, &message);
        },
        "remove" => {
            let cmd = if cmd == "xbps-install" {
                "xbps-remove"
            } else {
                cmd
            };
            let args: Vec<&str> = match cmd {
                "emerge" => vec!("-a", "-v", "-c", &package),
                "xbps-remove" => vec!(&package),
                _ => vec!("N/A"),
            };
            if args[0] == "N/A" {
                println!("You found a bug, it shouldn't be possible to reach this but I had to cover this to make rust happy.");
                exit(1);
            }
            let message = ["Could not remove ", &package].concat();
            run(cmd, &args, &message);
        },
        _ => {
            println!("Sorry, {} is not a supported operation right now.", operation);
            exit(1);
        }
    }
}

fn backend(package_manager: &str, package: &str, operation: &str) {
    match package_manager {
        "portage" => match_op(operation, "emerge", package),
        "xbps" => match_op(operation, "xbps-install", package),
        _ => {
            println!("Sorry, {} is not a supported package manager right now.", package_manager);
            exit(1);
        }
    }
}

pub fn manage(package_manager: &str, packages: &str, operation: &str) {
    let packages = packages.replace("\n", " ");
    let package_vector = packages.split(" ").collect::<Vec<&str>>();
    for i in package_vector {
        backend(&package_manager, &i.to_string(), &operation);
    }
}
