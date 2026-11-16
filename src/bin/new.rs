use std::{env, fs, path::Path};
use std::io::{stdin, stdout, Write};

const TEMPLATE: &str = include_str!("template.txt");

fn main() {
    //Get the day number from the user
    print!("Enter the day number: ");
    let _ = stdout().flush();
    let mut day = String::new();
    stdin().read_line(&mut day).expect("Couldn't read input");
    let day: u32 = day.trim().parse().expect("day must be a number");

    //Create the path to the new folder
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let dir_name = format!("day{day:02}");
    let dir = root.join("src\\bin").join(&dir_name);
    if dir.exists() {
        eprintln!("{} already exists", dir.display());
        return;
    }

    //Create the day's folder and files
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.rs"), TEMPLATE).unwrap();
    fs::write(dir.join("input.txt"), "").unwrap();
    fs::write(dir.join("test1.txt"), "").unwrap();
    println!("Created {}", dir.display());

    // Add the day to Cargo.toml. Changing this file makes RustRover reload the project.
    let manifest = root.join("Cargo.toml");
    let mut toml = fs::read_to_string(&manifest).unwrap();
    if !toml.contains(&format!("name = \"{dir_name}\"")) {
        if !toml.ends_with('\n') {
            toml.push('\n');
        }
        toml.push_str(&format!(
            "\n[[bin]]\nname = \"{dir_name}\"\npath = \"src/bin/{dir_name}/main.rs\"\n"
        ));
        fs::write(&manifest, toml).unwrap();
        println!("Added {dir_name} to Cargo.toml");
    }
}
