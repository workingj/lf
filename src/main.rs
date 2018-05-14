use std::env;
use std::fs::{DirEntry, read_dir};
use std::path::{Path};
use std::error::Error;
use std::process::{exit};
use std::str;
// use std::time::SystemTime;

/// config struct
#[derive(Debug)]
struct Config {
    lastedit: bool,     // last editet    -l, -last
    mbyte: bool,        // show megabytes -m, --mbyte
}

/// instantiate a config struct
fn build_config (l: bool, m: bool) -> Config {
    Config {
        lastedit: l,
        mbyte: m,
    }
}

fn main() {
    get_args();
    let mut files: Vec<DirEntry> = Vec::new();
    let mut folders: Vec<DirEntry> = Vec::new();

    let entries = read_dir(".");
    match entries {
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
        Ok(entries) => for entry in entries {
            if entry.as_ref().unwrap().metadata().unwrap().is_dir() == true {
                folders.push(entry.unwrap());
            }
            else {
                files.push(entry.unwrap());
            }
        }
    }
    
    for item in sort_name_ascending(folders) {
        println!("asc: {:?}", item.path().to_str());
    }
    for item in sort_name_descending(files){
        println!("asc: {:?}", item.path().to_str());
    }
}

/// creates a config struct for further procedures
/// exits if the args ```--help``` or ```--version``` are given
fn get_args () -> Config {
    let mut config = build_config(false, false);

    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == "-v" || arg == "--version"{
            println!("{}",VERSION);
            exit(0);
        }
        if arg == "-h" || arg == "--help" {
            println!("{}",HELP);
            exit(0);
        }
        if arg == "-l"||arg == "--last" {
            config.lastedit = true;
        }
        if arg == "-m"||arg == "--mbyte" {
            config.mbyte = true;
        }
    }
    println!("{:#?}", config );
    config
}

fn sort_name_ascending<'a>(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let mut min = &items[0];
            for i in 0..items.len() {
                position = 0;
                if min.file_name()
                    .to_str()
                    .unwrap_or("could not convert filename!")
                    .as_bytes() 
                    > items[i].file_name()
                    .to_str()
                    .unwrap_or("could not convert filename!")
                    .as_bytes() {
                        
                    min = &items[i];
                    position = i;
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

fn sort_name_descending<'a>(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let mut max = &items[0];
            for i in 0..items.len() {
                position = 0;
                if max.file_name()
                    .to_str()
                    .unwrap_or("could not convert filename!")
                    .as_bytes() 
                    < items[i].file_name()
                    .to_str()
                    .unwrap_or("could not convert filename!")
                    .as_bytes() {
                        
                    max = &items[i];
                    position = i;
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

// fn add_time() -> String {}

fn as_bytes(size: u64) -> String{
    let mut counter: u8 = 0;
    let mut bytes = String::new();
    let mut v = Vec::new();
    for c in size.to_string().chars().rev() {
        counter += 1;
        v.push(c);
        if counter == 3 && size > 999 || counter == 6 && size > 999999 {
            v.push('.');
        }
    };
    for c in v.iter().rev() {
        bytes.push(*c);
    }
    bytes
}

// fn as_mbytes() -> String{}

/*
fn run(dir: &Path) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                    .file_name()
                    .into_string()
                    .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            
            let metadata = entry.metadata()?;
            let size = metadata.len();
            // let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            // check if directory
            // let dir = file_or_dir(size);

            // formating of bytes
            let bytes = as_bytes(size);

            // println!(
                    // "{}{:>11}  {}  {}",
                    // "{}{:>11}  {}",
                    // dir,
                    // bytes,
                    // modified.format("%_d. %b %H:%M").to_string(),
                    // file_name,
            // );
        }
    }
    Ok(())
}
*/

static HELP: &'static str = "
lf - List Files/Folders 0.3
workingj <workingj@outlook.de>
Lists all files and folders in the current directory

USAGE:
    lf [FLAGS] 

FLAGS:
    -h, --help       Prints help information
    -v, --version    Prints version information
    -l  --last       Prints prints the last modified date";

static VERSION: &'static str = "
lf - List Files/Folders
workingj <workingj@outlook.de>
Version: 0.3";