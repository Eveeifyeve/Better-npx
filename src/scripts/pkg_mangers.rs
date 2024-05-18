



pub fn check_pkg_managers() -> Vec<String> {
    let pkg_managers = ["bun", "pnpm", "yarn", "npm"];
    let mut pkg_managers_installed: Vec<String> = Vec::new();

    // Check if each package manager is installed
    for pm in pkg_managers {
        let output = std::process::Command::new("which")
            .arg(pm)
            .output()
            .expect("Failed to execute command");
    
        let output_str = String::from_utf8_lossy(&output.stdout);
    
        if output_str.trim() != "" {
            pkg_managers_installed.push(pm.to_string());
        }
    }

    pkg_managers_installed
}