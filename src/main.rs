mod add;
mod pkg;
use std::io::prelude::Write;
use std::path::Path;

fn init(config_dir: &Path) {
    if !config_dir.exists() {
        std::fs::create_dir(config_dir).unwrap();
    }
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

    let raw_config_dir = [home, "/.config/dep-organizer".to_string()].concat();
    let config_dir = Path::new(&raw_config_dir);

    init(&config_dir);

    let args = std::env::args().collect::<Vec<String>>();
    let operation = &args[1];
    let (package_manager, dependent_package, dependee_packages) = 
        crate::add::log(operation, &raw_config_dir);

    if operation == "write" {
        let package_file_dir = [raw_config_dir, "/".to_string(), dependent_package].concat();
        println!("{}", package_file_dir);
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
