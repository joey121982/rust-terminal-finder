use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Type of either File(fi) or Folder(fo)
    #[arg(short, long)]
    ftype: String,

    /// File/Folder to search for
    #[arg(short, long)]
    name: String,

    /// Starting location from where to search
    #[arg(short, long)]
    location: Option<String>
}

fn main() {
    let defaultlocation: String = "/home".to_string();
    let args = Args::parse();
    let location: &str = &args.location.unwrap_or(defaultlocation);
    search(&args.name, &args.ftype, location);
}

fn search(target: &str, ftype: &str, location: &str) {
    let entries = fs::read_dir(location).expect("ERROR: Location does not exist or permission not granted.");

    for entry in entries {
        let path = entry.unwrap().path();
        if ftype == "fi" && path.is_dir() {
            continue;
        }
        if ftype == "fo" && path.is_file() {
            continue;
        }
        if path.file_name().unwrap() == target {
            println!("Found instance at: {}", location);
        }
        if ftype == "fo" && path.is_dir() {
            search(target, ftype, path.to_str().unwrap());
        }
    }
}