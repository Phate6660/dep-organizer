use std::io::{stdout, Write};
use std::process::Stdio;

fn run(cmd: &str, args: &[&str], message: &str) {
    let child = std::process::Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect(message);
    stdout().write_all(&child.stdout).unwrap();
}

fn backend(package_manager: &str, package: &str, operation: &str) {
    if package_manager == "portage"{
        if operation == "install" {
            let message = ["Could not install ", &package].concat();
            run("emerge", &["-a", "-t", "-v", &package], &message);
        } else if operation == "remove" {
            let message = ["Could not remove ", &package].concat();
            run("emerge", &["-a", "-v", "-c", &package], &message);
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
