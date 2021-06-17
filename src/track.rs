use std::io::prelude::Write;
use std::io::{Read, stdin};

pub fn log(operation: &str, package_manager: &str, raw_manager_dir: &str) -> (String, String) {
    println!("\nPlease enter the dependent package.");
    let mut dependent_package = String::new();
    stdin()
        .read_line(&mut dependent_package)
        .expect("Failed to read input.");

    let (_package_category, dependent_package) = 
        if package_manager.trim() == "portage" && dependent_package.contains('/') {
            let package_vector = dependent_package.split('/').collect::<Vec<&str>>();
            (package_vector[0], package_vector[1])
        } else {
            ("", dependent_package.as_str())
        };

    let mut dependee_packages = String::new();
    if operation == "install" || operation == "remove" {
        let deps_file = crate::format_and_trim(raw_manager_dir, dependent_package);
        let mut dependee_packages_file = std::fs::File::open(deps_file.trim())
            .expect("Unable to read the file.");
        dependee_packages_file.read_to_string(&mut dependee_packages)
            .expect("Could not read file.");
        dependee_packages = dependee_packages.replace("\n", " ").trim().to_string();
        (
            dependent_package.trim().to_string(),
            dependee_packages
        )
    } else {
        println!("\nPlease enter the dependnee packages.");
        stdin()
            .read_line(&mut dependee_packages)
            .expect("Failed to read input.");

        let dependee_packages = if dependee_packages.trim().contains(' ') {
            dependee_packages.replace(" ", "\n")
        } else {
            dependee_packages
        };

        (
            dependent_package.trim().to_string(),
            dependee_packages
        )
    }
}

pub fn write(raw_manager_dir: &str, dependent_package: &str, dependee_packages: &str) {
    let package_file_dir = crate::format_and_trim(raw_manager_dir, dependent_package);
    println!("Dependencies written to: {}", package_file_dir);
    let package_file_dir = std::path::Path::new(&package_file_dir);

    if package_file_dir.exists() {
        let mut package_file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(package_file_dir)
            .unwrap();
        write!(package_file, "{}", dependee_packages).unwrap();
    } else {
        let mut package_file = std::fs::File::create(package_file_dir).unwrap();
        package_file
            .write_all(dependee_packages.as_bytes())
            .unwrap();
    }
}
