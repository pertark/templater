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

    if args.len() == 1 {
        // fetch latest changes
        let output = Command::new("git")
            .current_dir(&config_dir)
            .args(&["pull", "origin"])
            .output()
            .expect("failed to fetch");
        println!("{}", String::from_utf8_lossy(&output.stdout));

        println!("Template Directory: {}" , config_dir.to_str().unwrap());
        return;        
    }

    // check if template exists
    let template = &args[1];
    let template_dir = config_dir.join(template);
    println!("Template: {}", template_dir.to_str().unwrap());
    if !template_dir.exists() {
        println!("Template doesn't exist");
        return;
    }


    // by default, copy template to current directory
    let default_target = env::current_dir().unwrap();
    let mut target_dirs = vec![default_target.as_path()];
    if args.len() > 2 {
        target_dirs = args[2..].iter().map(|x| Path::new(x)).collect();

        // create directories if they don't exist
        for dir in &target_dirs {
            if !dir.exists() {
                std::fs::create_dir_all(dir).expect("failed to create directory");
            }
        }
    }

    println!("Copying template...");
    for dir in target_dirs {
        copy_template(&template_dir, dir);
    }
    println!("Copying complete.");
}

fn copy_template(template_dir: &Path, target_dir: &Path) {
    for entry in std::fs::read_dir(template_dir).expect("failed to read directory") {
        let entry = entry.expect("failed to read entry");
        let path = entry.path();
        let file_name = path.file_name().unwrap();
        let target_path = target_dir.join(file_name);

        if path.is_dir() {
            std::fs::create_dir(&target_path).expect("failed to create directory");
            copy_template(&path, &target_path);
        } else {
            std::fs::copy(&path, &target_path).expect("failed to copy file");
        }
    }
}
