// lf
use std::fs::{DirEntry};
use std::error::Error;
use std::process::{exit};
pub mod functions;
use functions::*;

fn main() {
    let config = get_config_from_args();
    let content = get_files_folders(&config);

    match run(config, content) {
        Err(e) => {
            println!("{:?}",e);
            exit(1);
        },
        Ok(()) => exit(0),
    };
}

fn run(config: Config, content: Content) -> Result<(), Box<Error>> {
    let folders: Vec<DirEntry>;
    let mut files: Vec<DirEntry>;
    let output: Vec<String>;

    if config.size_asc == true {
        folders = content.0;
        files = sort_size_ascending(content.1);
        print!("size ascending:");
    } else if config.size_desc == true {
        folders = content.0;
        files = sort_size_descending(content.1);
        print!("size descending:");
    } else if config.name_asc == true {
        folders = sort_name_ascending(content.0);
        files = sort_name_ascending(content.1);
        print!("name ascending:");
    } else if config.name_desc == true {
        folders = sort_name_descending(content.0);
        files = sort_name_descending(content.1);
        print!("name descending:");
    } else if config.time_asc == true {
        folders = sort_time_ascending(content.0);
        files = sort_time_ascending(content.1);
        print!("time ascending:");
    } else if config.time_desc == true {
        folders = sort_time_descending(content.0);
        files = sort_time_descending(content.1);
        print!("time descending:");
    } else {
        folders = content.0;
        files = content.1;
    }

    if config.file_filter == true {
        println!("Files: {}", files.len());
        files = get_file_from_ending(files, &config.file_type);
        output = string_output_from_files(files);
    } else {
        println!(" {} Folders & {} Files", folders.len(), files.len());
        output = string_output_from_files_and_folders(folders, files);
    }
    for line in output {
        println!("{}", line);
    }

    return Ok(())
}