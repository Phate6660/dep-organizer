mod add;
mod pkg;
use std::io::prelude::Write;
use std::path::Path;

fn ensure_directory(dir: &Path) {
    if !dir.exists() {
        std::fs::create_dir(dir).unwrap();
    }
}

fn format_and_trim(part1: &str, part2: &str) -> String {
    let output = format!("{}/{}", part1, part2);
    output.trim().to_string()
}

fn main() {
    let user = std::env::var("USER").unwrap();
    let home = if user != "root" {
        std::env::var("HOME").unwrap()
    } else {
        // Most likely ran through sudo if the user is root.
        // TODO: Add more cases, for when sudo is not in use.
        let real_user = std::env::var("SUDO_USER").unwrap();
        ["/home/", real_user.as_str()].concat()
    };

    let raw_config_dir = format!("{}/.config/dep-organizer", home);
    let config_dir = Path::new(&raw_config_dir);
    ensure_directory(&config_dir);

    println!("Please enter your package manager.");
    let mut package_manager = String::new();
    std::io::stdin()
        .read_line(&mut package_manager)
        .expect("Failed to read input.");
    package_manager = package_manager.trim().to_string();

    let raw_manager_dir = format_and_trim(&raw_config_dir, &package_manager);
    let manager_dir = Path::new(&raw_manager_dir);
    ensure_directory(&manager_dir);

    let args = std::env::args().collect::<Vec<String>>();
    let operation = &args[1];
    let (dependent_package, dependee_packages) = 
        crate::add::log(operation, &package_manager, &raw_manager_dir);

    if operation == "write" {
        let package_file_dir = format_and_trim(&raw_manager_dir, &dependent_package);
        println!("Dependencies written to: {}", package_file_dir);
        let package_file_dir = Path::new(&package_file_dir);

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
    } else if operation == "install" || operation == "remove" {
        crate::pkg::manage(&package_manager, &dependee_packages, &operation);
    }
}
