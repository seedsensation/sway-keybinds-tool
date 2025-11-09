// read environment variable SWAY_CONFIG
use std::env;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use regex::Regex;

struct Keybind {
    keys: String,
    release: bool,
    execute: String,
}

fn help_text() -> String {
"\
This command is used to edit my config files in Sway.

Usage: keybinds [COMMAND]

- list - List all existing keybinds
- edit <id> <key / 'n/a'> <command / 'n/a'> <release?> - Edit an existing keybind
- new <key> <command> <release?> - Create a new keybind\
".to_string()

}

fn update_binds(binds: Vec<Keybind>) -> String {
    let mut output: String = String::new();
    for bind in binds {
        output.push_str(format!("bindsym {}{} exec {}\n",{
            if bind.release {
                "--release "
            } else { "" }
        },bind.keys, bind.execute).as_str());
    }
    output
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

    let contents = std::fs::read_to_string(&path)
        .expect("Failed to read file");
    let lines: Vec<&str> = contents
        .split("\n")
        .collect();

    let mut binds: Vec<Keybind> = Vec::new();
    
    for line in lines {
        if line.to_lowercase().starts_with("bindsym") {
            let re = Regex::new(r"bindsym (--release)? ?([^ ]*) exec (.*?)$").unwrap();
            let caps = re.captures(line).expect("not understood line");

            binds.push(Keybind {
                release: caps.get(1).is_some(),
                keys: caps.get(2).unwrap().as_str().to_string(),
                execute: caps.get(3).unwrap().as_str().to_string(),
            });
        }
    }

    if args.len() > 1 {
        match args[1].as_str() {
            "list" => {
                for (i,bind) in binds.iter().enumerate() {
                    println!("{}. Key: {}\n   Mode: {}\n   Command: {}",i+1,bind.keys,{
                        if bind.release { "On Release " } else { "On Press" }
                    },bind.execute);

                }
            },
            "new" => {
                let keys = args.get(2).expect("Missing key combination");
                println!("{}",keys);
                let exec = args.get(3).expect("Missing command to be executed");
                println!("{}",exec);

                let release = {
                    if args.get(4).is_some() {
                        let temp = args.get(4).unwrap().to_lowercase();
                        temp != "pressed" && temp != "false"
                    } else {
                        false
                    }
                };
                println!("{}",release);
                if keys.starts_with("+") || keys.len() == 0 {
                    println!("Please surround your key combinations with ''.");
                    return;
                }

                struct CheckDuplicates {
                    keys: String,
                    release: bool,
                }
                impl std::cmp::PartialEq for CheckDuplicates {
                    fn eq(&self, other: &Self) -> bool {
                        self.keys == other.keys &&
                        self.release == other.release
                    }
                }

                let mut binds_checker: Vec<CheckDuplicates> = Vec::new();
                for bind in &binds {
                    binds_checker.push(CheckDuplicates {
                        keys: bind.keys.to_lowercase(),
                        release: bind.release,
                    })
                }

                let checker = CheckDuplicates {
                    keys: keys.to_lowercase(),
                    release: release,
                };

                if binds_checker.contains(&checker) {
                    println!("This key combination already exists.");
                    return;
                }

                binds.push(Keybind {
                    keys: keys.to_string(),
                    release: release,
                    execute: exec.to_string()
                })


            },
            "edit" => {
                
                let binds_len = binds.len();
                let mut editable = binds.get_mut(
                    args.get(2).expect("Missing ID for editing")
                        .parse::<usize>().expect("ID should be a number") - 1)
                                    .expect(format!("ID {} out of range ({})",args.get(2).unwrap(), binds_len).as_str());
                let keys = args.get(3).expect("Missing key combination");
                println!("{}",keys);
                let na_text = "n/a".to_string();
                let exec = args.get(4).unwrap_or(&na_text);
                println!("{}",exec);
                let release = args.get(5).unwrap_or(&na_text);
                println!("{}",exec);

                editable.keys = if keys.to_lowercase() != "n/a" { keys.to_string() } else { editable.keys.clone() };
                editable.execute = if exec.to_lowercase() != "n/a" { exec.to_string() } else { editable.execute.clone() };
                editable.release = if release.to_lowercase() != "n/a" { release.parse::<bool>().unwrap_or(false) } else { editable.release };


                
            },
            _ => println!("{}",help_text()),
        }
    } else {
        println!("{}",help_text());
    }

    let mut data_file = File::create(path).expect("Could not open file");
    data_file.write(update_binds(binds).as_bytes()).expect("Write Failed");

    
}

