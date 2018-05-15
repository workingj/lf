use std::env;
use std::fs::{DirEntry, read_dir};
use std::process::{exit};

extern crate chrono;
use self::chrono::{DateTime, Local};


/// struct for configuring the run function
#[derive(Debug)]
pub struct Config {
    pub size_desc: bool,
    pub name_desc: bool,
    pub time_desc: bool,
}

/// instantiate a config struct
fn build_config (s: bool, n: bool, t: bool) -> Config {
    Config {
        size_desc: s,
        name_desc: n,
        time_desc: t,
    }
}

/// creates a config struct for further procedures
/// exits if the args ```--help``` or ```--version``` are given
pub fn get_args () -> Config {
    let mut config = build_config(false, false, false);

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
        if arg == "-s"||arg == "--size-desc" {
            config.size_desc = true;
        }
        if arg == "-n"||arg == "--name-desc" {
            config.name_desc = true;
        }
        if arg == "-t"||arg == "--time-desc" {
            config.time_desc = true;
        }
    }
    // println!("{:#?}", config );
    config
}

/// Container for the filesystem entries
pub struct Content (pub Vec<DirEntry>, pub Vec<DirEntry>);


pub fn get_files_folders() -> Content {
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
    Content(folders, files)
}

pub fn sort_size_ascending (mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let min = get_size(&items[0]);
            for i in 0..items.len() {
                position = 0;
                if min > get_size(&items[i]) {                        
                    position = i;
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

pub fn sort_size_descending (mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let mut min = get_size(&items[0]);
            position = 0;
            for i in 0..items.len() {
                if min < get_size(&items[i]) {                        
                    position = i;
                    min = get_size(&items[i]);
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

pub fn sort_name_ascending (mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let min = &items[0];
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
                        
                    position = i;
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

pub fn sort_name_descending (mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let max = &items[0];
            for i in 0..items.len() {
                position = 0;
                if max.file_name()
                    .to_str()
                    .expect("could not convert filename!")
                    .as_bytes() 
                    < items[i].file_name()
                    .to_str()
                    .expect("could not convert filename!")
                    .as_bytes() {
                        
                    position = i;
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

pub fn sort_time_ascending (mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize;

    while items.len() > 0 {
        {
            position = 0;
            let mut min = get_secs(&items[0]);
            for i in 0..items.len() {
                if min > get_secs(&items[i]) {
                    position = i;
                    min = get_secs(&items[i]);
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

pub fn sort_time_descending (mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize;

    while items.len() > 0 {
        {
            position = 0;
            let mut min = get_secs(&items[0]);
            for i in 0..items.len() {
                if min > get_secs(&items[i]) {
                    position = i;
                    min = get_secs(&items[i]);
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

fn get_secs(entry: &DirEntry) -> u64 {
    entry.metadata()
        .expect("could not read metadata")
        .modified()
        .expect("could not read metadata.modified")
        .elapsed()
        .expect("could not get duration")
        .as_secs()
}

fn get_size(entry: &DirEntry) -> u64 {
    entry.metadata()
    .expect("could not read metadata")
    .len()
}

/// formating the files size in bytes
pub fn as_formated_bytes(size: u64) -> String{
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

pub fn merge(folders: Vec<DirEntry>, files: Vec<DirEntry>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    
    for folder in folders {
        let modified: DateTime<Local> = DateTime::from(folder.metadata().unwrap().modified().unwrap());
        let time = modified.format("%D %H:%M").to_string();
        let size = " ";
        let name = folder.path().as_path().file_name().unwrap().to_owned().into_string().unwrap();
        output.push(format!(" {} D {:>11}  {}",time, size, name));
    }
    for file in files {
        let modified: DateTime<Local> = DateTime::from(file.metadata().unwrap().modified().unwrap());
        let time = modified.format("%D %H:%M").to_string();
        let size = as_formated_bytes(file.metadata().unwrap().len());
        let mut name = file.path().as_path().file_name().unwrap().to_owned().into_string().unwrap();
        output.push(format!(" {} F {:>11}  {}",time, size, name));
    }

    output
}

static HELP: &'static str = "
lf - List Files/Folders 0.3
workingj <workingj@outlook.de>
Lists all files and folders in the current directory

USAGE:
    lf [FLAG] 

FLAGS:
    -h, --help        Prints help information
    -v, --version     Prints version information
    -s, --size-desc   Sorts entries size descending
    -n, --name-desc   Sorts entries name ascending
    -t, --time-desc   Sorts entries time desending";

static VERSION: &'static str = "
lf - List Files/Folders
workingj <workingj@outlook.de>
Version: 0.3";