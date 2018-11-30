// lf
use std::fs::{DirEntry};
// use std::path::Path;
use std::error::Error;
use std::process::{exit};
// use std::str;
// use std::time::SystemTime;

pub mod functions;
use functions::*;

fn main() {
    let config = get_args();
    let content = get_files_folders();

    match run(config, content) {
        Err(e) => {
            println!("{:?}",e);
            exit(1);
        },
        Ok(()) => exit(0),
    };
}

fn run(args: Config, content: Content) -> Result<(), Box<Error>> {
    let mut folders: Vec<DirEntry> = Vec::new();
    let mut files: Vec<DirEntry> = Vec::new();
    let mut output: Vec<String> = Vec::new();
    
    if args.size_desc == true {
        folders = content.0;
        files = sort_size_descending(content.1);
        println!("size descending");
    } else if args.name_desc == true {
        folders = sort_name_descending(content.0);
        files = sort_name_descending(content.1);
        println!("name descending");
    } else if args.time_desc == true {
        folders = sort_time_descending(content.0);
        files = sort_time_descending(content.1);
        println!("time descending");
    } else {
        folders = content.0;
        files = content.1;
    }

    output = merge(folders, files);

    for line in output {
        println!("{}", line);
    }

    return Ok(())
}