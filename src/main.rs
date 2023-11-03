use std::env;
use std::path::Path;
use std::process::Command;

#[cfg(target_os="windows")]
fn get_config_dir() -> String {
    let config_dir = env::var("LOCALAPPDATA").unwrap();
    config_dir
}

#[cfg(target_os="linux")]
fn get_config_dir() -> String {
    let config_dir = env::var("XDG_CONFIG_HOME").unwrap_or(format!("{}/.config", env::var("HOME").unwrap()));
    config_dir
}

fn main() {
    // better config file later
    let url = include_str!("template_url");

    // replace with something more sophisticated later
    let args: Vec<String> = env::args().collect();

    // get config dir
    let config_dir = get_config_dir();
    let config_dir = Path::new(&config_dir);
    let config_dir = config_dir.join("templater");

    // check if repository exists before cloning
    if !config_dir.exists() {
        println!("Repository doesn't exist yet, cloning...");
        // Repository::clone(url, &config_dir).expect("failed to clone");
        let output = Command::new("git")
            .args(&["clone", url, &config_dir.to_str().unwrap()])
            .output()
            .expect("failed to clone");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // fetch latest changes
    let output = Command::new("git")
        .current_dir(&config_dir)
        .args(&["pull", "origin"])
        .output()
        .expect("failed to fetch");
    println!("{}", String::from_utf8_lossy(&output.stdout));

    // check if template exists
    let template = &args[1];
    let template_dir = config_dir.join(template);
    println!("Template: {}", template_dir.to_str().unwrap());
    if !template_dir.exists() {
        println!("Template doesn't exist");
        return;
    }

    // copy template to current directory
    println!("Copying template...");
    let current_dir = env::current_dir().unwrap();

    // for each file in template_dir, copy to current_dir
    for entry in template_dir.read_dir().expect("read_dir call failed") {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        let current_dir = current_dir.join(file_name);
        println!("Copying {} to {}", path.to_str().unwrap(), current_dir.to_str().unwrap());
        std::fs::copy(path, current_dir).expect("failed to copy");
    }
    
}
