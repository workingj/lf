use std::error::Error;
use std::ffi::OsStr;
use std::fs::{read_dir, DirEntry, FileType};
use std::path::PathBuf;
use std::process::exit;
extern crate chrono;
use self::chrono::{DateTime, Local};
use error::my_error;

#[allow(dead_code)]
fn get_file_filter(entry: &DirEntry) -> FileType {
    entry
        .metadata()
        .expect("could not read metadata")
        .file_type()
}

/// struct for configuring the run function
#[derive(Debug)]
pub struct Config {
    pub size_asc: bool,
    pub size_desc: bool,
    pub name_asc: bool,
    pub name_desc: bool,
    pub time_asc: bool,
    pub time_desc: bool,
    pub file_filter: bool,
    pub file_type: String,
    pub path: PathBuf,
}

impl Config {
    fn new() -> Config {
        Config {
            size_asc: false,
            size_desc: false,
            time_asc: false,
            time_desc: false,
            name_asc: false,
            name_desc: false,
            file_filter: false,
            file_type: String::new(),
            path: PathBuf::new(),
        }
    }
}

/// Building the `Config` for the main run function
pub fn get_config_from_args(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
    let mut config = Config::new();
    for arg in &args {
        if arg == "-v" || arg == "--version" {
            println!("{}", VERSION);
            exit(0);
        }
        if arg == "-h" || arg == "--help" {
            println!("{}", MAN_PAGE);
            exit(0);
        }
        if arg == "-s" || arg == "--size-asc" {
            config.size_asc = true;
            break;
        }
        if arg == "-sd" || arg == "--size-desc" {
            config.size_desc = true;
            break;
        }
        if arg == "-n" || arg == "--name-asc" {
            config.name_asc = true;
            break;
        }
        if arg == "-nd" || arg == "--name-desc" {
            config.name_desc = true;
            break;
        }
        if arg == "-t" || arg == "--time-asc" {
            config.time_asc = true;
            break;
        }
        if arg == "-td" || arg == "--time-desc" {
            config.time_desc = true;
            break;
        }

        let mut path = PathBuf::new();

        // check if path is given
        if arg.chars().nth(0).unwrap() == '/' || arg.chars().nth(0).unwrap() == '\\' {
            path.push(&arg[1..]);
            if path.exists() {
                if path.is_file() {
                    return Result::Err(Box::new(my_error("Path is a file!".to_string())));
                } else {
                    config.path.push(path);
                }
            } else {
                return Result::Err(Box::new(my_error("Path does not exist!".to_string())));
            }
        //  check for file extension
        } else if arg.chars().nth(0).unwrap() == '.' {
            path.push(&arg);
                dbg!(path.exists());
                dbg!(path.is_file());
            if path.exists() {
                return Result::Err(Box::new(my_error("Given path is not valid!".to_string())));
            } else if path.is_dir() {
                if config.path.to_str() == Some("") {
                    config.path.push(path);
                }
            } else if path.is_file() {
                return Result::Err(Box::new(my_error("Given argument is a file!".to_string())));
            // for fileextension
            } else {
                config.file_filter = true;
                let path = path.display().to_string();
                config.file_type = path[1..].to_string();
            }
        // check for normal input
        } else if arg.chars().nth(0).unwrap() != '.'
            || arg.chars().nth(0).unwrap() != '/'
            || arg.chars().nth(0).unwrap() != '\\'
        {
            path.push(&arg);
            if !path.exists() {
                return Result::Err(Box::new(my_error("Given path is not valid!".to_string())));
            } else if path.is_dir() {
                config.path.push(path);
            } else if path.is_file() {
                return Result::Err(Box::new(my_error("Given argument is a file!".to_string())));
            }
        }
    }
    Ok(config)
}

/// Container for the filesystem entries
// pub struct Content(pub Vec<DirEntry>, pub Vec<DirEntry>);
pub struct Content {
    pub files: Vec<DirEntry>,
    pub folders: Vec<DirEntry>,
}
fn build_content(files: Vec<DirEntry>, folders: Vec<DirEntry>) -> Content {
    Content { folders, files }
}

/// Returns the Content `struct` from the read directory
/// ```
/// return Content(folders, files)
/// ```
pub fn get_folders_files(config: &Config) -> Content {
    let mut files: Vec<DirEntry> = Vec::new();
    let mut folders: Vec<DirEntry> = Vec::new();

    let entries = if config.path.to_str() == Some("") {
        read_dir(".")
    } else {
        read_dir(format!("./{}", config.path.to_str().unwrap()))
    };

    match entries {
        Ok(entries) => {
            for entry in entries {
                if entry.as_ref().unwrap().metadata().unwrap().is_dir() {
                    folders.push(entry.unwrap());
                } else {
                    files.push(entry.unwrap());
                }
            }
        }
        Err(e) => {
            println!("in get ff)) {}", e);
            exit(1);
        }
    }
    build_content(files, folders)
}

/// get all DirEntrys from File-List that match the given file extension
pub fn get_file_from_ending(mut items: Vec<DirEntry>, file_type: &str) -> Vec<DirEntry> {
    items.retain(|item| {
        file_type
            == item
                .path()
                .extension()
                .unwrap_or_else(|| OsStr::new(""))
                .to_str()
                .unwrap()
    });
    items
}

/// formating the files size bytedisplay
pub fn as_formated_bytes(size: u64) -> String {
    // let mut counter: u8 = 0;
    let mut bytes = String::new();
    let mut v = Vec::new();
    let file_len = size.to_string();
    let len = file_len.len() + 1;
        dbg!(&file_len, &len);

    for (c, counter) in file_len.chars().rev().zip(1..len) {
        dbg!(&c, &counter);
        // counter += 1;
        v.push(c);
        if counter == 3 && size > 999
            || counter == 6 && size > 999_999
            || counter == 9 && size > 999_999_999
        {
            v.push('.');
        }
    }
    for c in v.iter().rev() {
        bytes.push(*c);
    }
    bytes
}

/// Create output Strings for files and folders
pub fn string_output_from_files_and_folders(
    folders: Vec<DirEntry>,
    files: Vec<DirEntry>,
) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for folder in folders {
        let modified: DateTime<Local> =
            DateTime::from(folder.metadata().unwrap().modified().unwrap());
        let time = modified.format("%Y/%m/%d %H:%M:%S").to_string();
        let size = "<dir>";
        let name = folder
            .path()
            .as_path()
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        output.push(format!(" {} {:>12}  {}", time, size, name));
    }

    for file in files {
        let modified: DateTime<Local> =
            DateTime::from(file.metadata().unwrap().modified().unwrap());
        let time = modified.format("%Y/%m/%d %H:%M:%S").to_string();
        let size = as_formated_bytes(file.metadata().unwrap().len());
        let name = file
            .path()
            .as_path()
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        output.push(format!(" {} {:>12}  {}", time, size, name));
    }

    output
}

/// Create string output only from files
pub fn string_output_from_files(files: Vec<DirEntry>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for file in files {
        let modified: DateTime<Local> =
            DateTime::from(file.metadata().unwrap().modified().unwrap());
        let time = modified.format("%y/%m/%d %H:%M:%S").to_string();
        let size = as_formated_bytes(file.metadata().unwrap().len());
        let name = file
            .path()
            .as_path()
            .file_name()
            .unwrap()
            .to_owned()
            .into_string()
            .unwrap();
        output.push(format!(" {} {:>12}  {}", time, size, name));
    }

    output
}

static MAN_PAGE: &str = r#"
NAME:
    lf - List Files/Folders 1.0.0

DESCRIPTION:
    Lists all files and folders in the current directory

USAGE:
    lf [folder or path] [ -h  -v  -s  -n  -t ] [.file-extension]

OPTIONS:
    folder or path     Lists all entries in the given folder or path. 
                       Has to be a subfolder of the current path. 
    -h,  --help        Print help information
    -v,  --version     Print version information
    -s,  --size-asc    Sort size ascending
    -sd, --size-desc   Sort size descending
    -n,  --name-asc    Sort name ascending
    -nd, --name-desc   Sort name descending
    -t,  --time-asc    Sort time asending
    -td, --time-desc   Sort time desending
    .file-extension    List only files with given file-extension.
"#;

static VERSION: &str = "
lf - List Files/Folders
VERSION: 1.0.0";
