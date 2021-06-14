use std::io::{Read, stdin};

pub fn log(operation: &str, raw_config_dir: &str) -> (String, String, String) {
    println!("Please enter your package manager.");
    let mut package_manager = String::new();
    stdin()
        .read_line(&mut package_manager)
        .expect("Failed to read input.");

    println!("\nPlease enter the dependent package.");
    let mut dependent_package = String::new();
    stdin()
        .read_line(&mut dependent_package)
        .expect("Failed to read input.");

    let (_package_category, dependent_package) = 
        if package_manager.trim() == "portage" && dependent_package.contains("/") {
            let package_vector = dependent_package.split("/").collect::<Vec<&str>>();
            (package_vector[0], package_vector[1])
        } else {
            ("", dependent_package.as_str())
        };

    let mut dependee_packages = String::new();
    if operation == "install" || operation == "remove" {
        let deps_file = [raw_config_dir, "/", dependent_package].concat();
        let mut dependee_packages_file = std::fs::File::open(deps_file.trim())
            .expect("Unable to read the file.");
        dependee_packages_file.read_to_string(&mut dependee_packages)
            .expect("Could not read file.");
        dependee_packages = dependee_packages.replace("\n", " ").trim().to_string();
        (
            package_manager.trim().to_string(),
            dependent_package.trim().to_string(),
            dependee_packages
        )
    } else {
        println!("\nPlease enter the dependnee packages.");
        stdin()
            .read_line(&mut dependee_packages)
            .expect("Failed to read input.");

        let dependee_packages = if dependee_packages.trim().contains(" ") {
            dependee_packages.replace(" ", "\n")
        } else {
            dependee_packages
        };

        (
            package_manager.trim().to_string(),
            dependent_package.trim().to_string(),
            dependee_packages.to_string()
        )
    }
}

