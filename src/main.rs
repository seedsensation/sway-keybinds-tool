// read environment variable SWAY_CONFIG
use std::env;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use regex::Regex;

struct Keybind {
    keys: String,
    execute: String,
}

fn help_text() -> String {
"
This command is used to edit my config files in Sway.

Usage: sway-keys [COMMAND]

- list      List all existing keybinds
- edit      Edit an existing keybind
- new       Create a new keybind
".to_string()

}
fn main() {
    let key = "SWAYCONFIG";
    let path_string: String;
    let args: Vec<String> = env::args().collect();
    
    match env::var(&key) {
        Ok(val) => path_string = val,
        Err(_) => panic!("Unknown environment variable {}",key)
    }

    let mut path = Path::new(path_string.as_str()).to_path_buf();
    if !path.exists() {
        panic!("SWAYCONFIG points to invalid file path");
    }

    path.push("keybinds");
    if !path.exists() {
        File::create(&path).unwrap().write_all(b"# Keybinds\n")
                                    .expect("Failed to create file");
    }

    let contents = std::fs::read_to_string(path)
        .expect("Failed to read file");
    let lines: Vec<&str> = contents
        .split("\n")
        .collect();

    let mut binds: Vec<Keybind> = Vec::new();
    
    for line in lines {
        if line.to_lowercase().starts_with("bindsym") {
            let re = Regex::new(r"bindsym ([^ ]*) exec (.*?)$").unwrap();
            let caps = re.captures(line).expect("not understood line");

            binds.push(Keybind {
                keys: caps.get(1).unwrap().as_str().to_string(),
                execute: caps.get(2).unwrap().as_str().to_string(),
            });
        }
    }

    if args.len() > 1 {
        match args[1].as_str() {
            "list" => {
                for (i,bind) in binds.iter().enumerate() {
                    println!("{}. Key: {}\n   Command: {}",i,bind.keys,bind.execute);

                }
            },
            _ => println!("{}",help_text()),
        }
    } else {
        println!("{}",help_text());
    }


    
}

