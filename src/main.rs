mod pkg;
mod track;
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

fn print_help_and_exit() {
    println!("
Current operations supported:
- ammdend (remove any duplicate dependencies for a dependent package)
- remove (stop tracking dependencies for a program)
- write (track dependencies for a dependent package)
- install (bulk install dependencies for a dependent package)
- uninstall (bulk uninstall dependencies for a dependent package)
    ");
    std::process::exit(1);
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let na = "N/A".to_string();
    let operation = args.get(1).unwrap_or(&na);
    let operation_arg = args.get(2).unwrap_or(&na); // only -a (automatically ammend) for  write is available

    if operation == "N/A" {
        print_help_and_exit();
    }

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
    ensure_directory(config_dir);

    println!("Please enter your package manager.");
    let mut package_manager = String::new();
    std::io::stdin()
        .read_line(&mut package_manager)
        .expect("Failed to read input.");
    package_manager = package_manager.trim().to_string();

    let raw_manager_dir = format_and_trim(&raw_config_dir, &package_manager);
    let manager_dir = Path::new(&raw_manager_dir);
    ensure_directory(manager_dir);

    let (dependent_package, dependee_packages) = 
        track::log(operation, &package_manager, &raw_manager_dir);

    match operation.as_str() {
        "ammend" => track::ammend(&raw_manager_dir, &dependent_package),
        "remove" => track::remove(&raw_manager_dir, &dependent_package),
        "write" => {
            track::write(&raw_manager_dir, &dependent_package, &dependee_packages);
            if operation_arg == "-a" {
                track::ammend(&raw_manager_dir, &dependent_package);
            }
        },
        "install" | "uninstall" => pkg::manage(&package_manager, &dependee_packages, operation),
        _ => print_help_and_exit(),
    }
}
