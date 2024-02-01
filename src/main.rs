use async_recursion::async_recursion;
use clap::Parser;
use std::fs;
use tokio::task;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Type of either File(fi) or Folder(fo)
    #[arg(short, long)]
    ftype: Option<String>,

    /// File/Folder to search for
    #[arg(short, long)]
    name: String,

    /// Starting location from where to search
    #[arg(short, long)]
    location: Option<String>,
}

#[tokio::main]
async fn main() {
    let default_location: String = "/home".to_string();
    let args = Args::parse();
    let location: &str = &args.location.unwrap_or(default_location);

    let rftype: &str = &args.ftype.unwrap_or("all".to_string());
    search(&args.name, &rftype, location).await;
}

async fn search(target: &str, ftype: &str, location: &str) {
    let entries = fs::read_dir(location).expect("ERROR: Location does not exist or permission not granted.");

    let mut tasks = Vec::new();

    for entry in entries {
        let target = target.to_owned();
        let ftype = ftype.to_owned();
        let location = entry.unwrap().path().to_str().unwrap().to_owned();

        let task = task::spawn(async move {
            search_in_directory(&target, &ftype, &location).await;
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }
}

#[async_recursion]
async fn search_in_directory(target: &str, ftype: &str, location: &str) {
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
        if ( ftype == "fo" || ftype == "all" ) && path.is_dir() {
            search_in_directory(target, ftype, path.to_str().unwrap()).await;
        }
    }
}
