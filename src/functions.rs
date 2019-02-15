use std::env;
use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry, FileType};
use std::process::exit;

extern crate chrono;
use self::chrono::{DateTime, Local};

/// struct for configuring the run function
#[derive(Debug)]
pub struct Config {
    pub size_desc: bool,
    pub name_desc: bool,
    pub time_desc: bool,
    pub file_filter: bool,
    pub file_type: String,
}

/// instantiate a config struct
fn build_config(s: bool, n: bool, t: bool, f: bool, e: String) -> Config {
    Config {
        size_desc: s,
        name_desc: n,
        time_desc: t,
        file_filter: f,
        file_type: e,
    }
}

/// creates a config struct for further procedures
/// exits if the args ```--help``` or ```--version``` are given
pub fn get_config_from_args() -> Config {
    let mut config = build_config(false, false, false, false, String::new());
    let args: Vec<String> = env::args().collect();
    let mut i: usize = 0;

    // TODO change config builder to exit when invalid is given
    // TODO eventualy fully remove -f and simply appand extension optionally
    // if args.len() > 2 {
    //     config.file_filter = true;
    // }

    for arg in &args {
        if arg == "-v" || arg == "--version" {
            println!("{}", VERSION);
            exit(0);
        }
        if arg == "-h" || arg == "--help" {
            println!("{}", MAN_PAGE);
            exit(0);
        }
        //  check for file extension
        if arg.chars().nth(0).unwrap() == '.' {
            config.file_filter = true;
            config.file_type = arg[1..].to_string();
        }
        if arg == "-s" || arg == "--size-desc" {
            config.size_desc = true;
        }
        if arg == "-n" || arg == "--name-desc" {
            config.name_desc = true;
        }
        if arg == "-t" || arg == "--time-desc" {
            config.time_desc = true;
        }


        i += 1;
    }

    config
}

/// Container for the filesystem entries
pub struct Content(pub Vec<DirEntry>, pub Vec<DirEntry>);

pub fn get_files_folders() -> Content {
    let mut files: Vec<DirEntry> = Vec::new();
    let mut folders: Vec<DirEntry> = Vec::new();

    let entries = read_dir(".");
    match entries {
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
        Ok(entries) => {
            for entry in entries {
                if entry.as_ref().unwrap().metadata().unwrap().is_dir() == true {
                    folders.push(entry.unwrap());
                } else {
                    files.push(entry.unwrap());
                }
            }
        }
    }
    Content(folders, files)
}

pub fn sort_size_ascending(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
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

pub fn sort_size_descending(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize;

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

pub fn sort_name_ascending(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let min = &items[0];
            for i in 0..items.len() {
                position = 0;
                if min
                    .file_name()
                    .to_str()
                    .unwrap_or("could not convert filename!")
                    .as_bytes()
                    > items[i]
                        .file_name()
                        .to_str()
                        .unwrap_or("could not convert filename!")
                        .as_bytes()
                {
                    position = i;
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

pub fn sort_name_descending(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let mut position: usize = 0;

    while items.len() > 0 {
        {
            let max = &items[0];
            for i in 0..items.len() {
                position = 0;
                if max
                    .file_name()
                    .to_str()
                    .expect("could not convert filename!")
                    .as_bytes()
                    < items[i]
                        .file_name()
                        .to_str()
                        .expect("could not convert filename!")
                        .as_bytes()
                {
                    position = i;
                }
            }
        }
        out.push(items.remove(position));
    }
    out
}

pub fn sort_time_ascending(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
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

pub fn sort_time_descending(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
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

// #[allow(unused)]
// pub fn get_file_from_ending(mut items: Vec<DirEntry>) -> Vec<DirEntry> {
//     let mut out: Vec<DirEntry> = Vec::new();
//     let mut search_type: FileType;
//     let args: Vec<String> = env::args().collect();
//     let default = OsStr::new("");

//     for item in items {
//         if args[2] == item.path().extension().unwrap_or(default).to_str().unwrap() {
//             out.push(item);
//         }
//     }
//     out
// }
// #[allow(unused)]
// get all DirEntrys from File-List that match the given file extension
pub fn get_file_from_ending(items: Vec<DirEntry>, file_type: &str) -> Vec<DirEntry> {
    let mut out: Vec<DirEntry> = Vec::new();
    let default = OsStr::new("");

    let file_type: String = String::from(file_type);

    for item in items {
        if file_type == item.path().extension().unwrap_or(default).to_str().unwrap() {
            out.push(item);
        }
    }
    out
}

#[allow(dead_code)]
fn get_file_fiter(entry: &DirEntry) -> FileType {
    entry
        .metadata()
        .expect("could not read metadata")
        .file_type()
}

fn get_secs(entry: &DirEntry) -> u64 {
    entry
        .metadata()
        .expect("could not read metadata")
        .modified()
        .expect("could not read metadata.modified")
        .elapsed()
        .expect("could not get duration")
        .as_secs()
}

fn get_size(entry: &DirEntry) -> u64 {
    entry.metadata().expect("could not read metadata").len()
}

/// formating the files size in bytes
pub fn as_formated_bytes(size: u64) -> String {
    let mut counter: u8 = 0;
    let mut bytes = String::new();
    let mut v = Vec::new();
    for c in size.to_string().chars().rev() {
        counter += 1;
        v.push(c);
        if counter == 3 && size > 999 || counter == 6 && size > 999999 {
            v.push('.');
        }
    }
    for c in v.iter().rev() {
        bytes.push(*c);
    }
    bytes
}

pub fn string_output_from_files_and_folders(folders: Vec<DirEntry>,files: Vec<DirEntry>,) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for folder in folders {
        let modified: DateTime<Local> =
            DateTime::from(folder.metadata().unwrap().modified().unwrap());
        let time = modified.format("%D %H:%M").to_string();
        let size = " ";
        let name = folder
            .path()
            .as_path()
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        output.push(format!(" {} D {:>11}  {}", time, size, name));
    }
    for file in files {
        let modified: DateTime<Local> =
            DateTime::from(file.metadata().unwrap().modified().unwrap());
        let time = modified.format("%D %H:%M").to_string();
        let size = as_formated_bytes(file.metadata().unwrap().len());
        let mut name = file
            .path()
            .as_path()
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        output.push(format!(" {} F {:>11}  {}", time, size, name));
    }

    output
}
pub fn string_output_from_files(files: Vec<DirEntry>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for file in files {
        let modified: DateTime<Local> =
            DateTime::from(file.metadata().unwrap().modified().unwrap());
        let time = modified.format("%D %H:%M").to_string();
        let size = as_formated_bytes(file.metadata().unwrap().len());
        let mut name = file
            .path()
            .as_path()
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        output.push(format!(" {} F {:>11}  {}", time, size, name));
    }

    output
}

static MAN_PAGE: &'static str = r#"
NAME:
    lf - List Files/Folders 0.6.1

DESCRIPTION:
    Lists all files and folders in the current directory

USAGE:
    lf [ -h | -v | -s | -n | -t ] [.FILEEXTENSION]

OPTIONS:
    -h, --help        Print help information
    -v, --version     Print version information
    -s, --size-desc   Sort entries size descending
    -n, --name-desc   Sort entries name ascending
    -t, --time-desc   Sort entries time desending
    .[file extension] List only files with given file extension."#;

static VERSION: &'static str = "
lf - List Files/Folders
VERSION: 0.6.1";
