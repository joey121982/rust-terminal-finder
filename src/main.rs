use std::{env, fs};
use std::error::Error;
use std::process::exit;
use dirs::home_dir;

fn parse_args(args: Vec<String>, target: &mut String, location: &mut String, extra_args: &mut Vec<String>) {
    if args.len() < 2 || args[1] == "-help" {
        println!(
            "\n\nSimple command line script that finds specified file/folder and prints its location.\n\n\n\
            USAGE: rust-terminal-finder.exe {{target}} {{start_location}} ...-extra_args\n\n"
        );
        exit(0);
    }

    for (index, arg) in args.iter().enumerate() {
        if index == 0 {
            continue;
        }
        if (arg.chars().collect::<Vec<char>>())[0] != '-' {
            if target == "@NOT_SPECIFIED" {
                *target = arg.to_string();
            } else {
                *location = arg.to_string();
            }
        } else {
            extra_args.push(arg.to_string());
        }
    }
}

fn search(target: &str, location: &str, args: Vec<String>, mut count: i32) -> Result<(), Box<dyn Error>> {
    let mut maxcount: i32 = 100;

    for arg in args.clone() {
        let narg:Vec<char> = arg.chars().collect();
        if narg[1] == 'm' && narg[2] == 'a' && narg[3] == 'x' {
            if narg.len() > 5 {
                maxcount = narg[4].to_digit(10).unwrap() as i32 * 10 + narg[5].to_digit(10).unwrap() as i32
            } else {
                maxcount = narg[4].to_digit(10).unwrap() as i32;
            }
        }
    }

    let entries = fs::read_dir(location)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if file_name == target {
                    count += 1;
                    println!("Found file: {}", path.to_string_lossy());
                }
            }
        } else if path.is_dir() {
            if let Some(file_name) = path.file_name() {
                if file_name == target {
                    count += 1;
                    println!("Found directory: {}", path.to_string_lossy());
                }
            }
            if count < maxcount {
                search(target, &path.to_string_lossy(), args.clone(), count)?;
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc: usize = args.len();
    let mut target: String = String::from("@NOT_SPECIFIED");
    let mut extra_args: Vec<String> = vec![];
    let mut location: String = "".to_string();

    if let Some(home) = home_dir() {
        location = home.to_str().unwrap().to_string();
    }

    parse_args(args, &mut target, &mut location, &mut extra_args);

    if argc == 1 || target == "@NOT_SPECIFIED" {
        println!("\nIncorrect usage, see -help for help.");
    } else if argc == 2 {
        println!("\nLooking for \"{}\", starting at default location: \"{}\"...", target, location);
    } else if argc == 3 {
        println!("\nLooking for \"{}\", starting at \"{}\"...", target, location);
    } else if argc > 3 {
        print!("\nLooking for \"{}\", starting at \"{}\", with extra args: ", target, location);
        for arg in extra_args.clone() {
            print!("{} ", arg);
        }
        println!("...");
    }
    println!();
    if let Err(err) = search(&target, &location, extra_args, 0) {
        eprintln!("Error: {}", err);
    } else {
        println!("\nFinished search. Exiting...\n\n");
    }
}
