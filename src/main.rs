// lf
use std::env;
use std::error::Error;
use std::fs::DirEntry;
use std::process::exit;
pub mod error;
pub mod functions;
pub mod sort;
use functions::*;
use sort::*;

fn main() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let config = get_config_from_args(args)?;
    let content = get_folders_files(&config);

    match run(config, content) {
        Err(e) => {
            println!("RUN ERROR: {}", e);
            exit(1);
        }
        Ok(()) => exit(0),
    };
}

fn run(config: Config, mut content: Content) -> Result<(), Box<Error>> {
    sort_name_ascending(&mut content.files)?;
    sort_name_ascending(&mut content.folders)?;

    if config.name_asc {
        println!("name ascending:");
        sort_name_ascending(&mut content.files)?;
        sort_name_ascending(&mut content.folders)?;
    } else if config.name_desc {
        println!("name descending:");
        sort_name_descending(&mut content.files)?;
        sort_name_descending(&mut content.folders)?;
    } else if config.size_asc {
        println!("size ascending:");
        sort_size_ascending(&mut content.files)?;
    } else if config.size_desc {
        println!("size descending:");
        sort_size_descending(&mut content.files)?;
    } else if config.time_asc {
        println!("time ascending:");
        sort_time_ascending(&mut content.files)?;
        sort_time_ascending(&mut content.folders)?;
    } else if config.time_desc {
        println!("time descending:");
        sort_time_descending(&mut content.files)?;
        sort_time_descending(&mut content.folders)?;
    }
    let mut files: Vec<DirEntry>;
    let output: Vec<String>;

    print!("{}", env::current_dir().unwrap().display());
    if config.file_filter {
        println!(
            "/{}\t *.{} | Files: {}",
            config.path.to_str().unwrap(),
            config.file_type,
            content.files.len()
        );
        files = get_file_from_ending(content.files, &config.file_type);
        output = string_output_from_files(files);
    } else {
        println!(
            "/{}\t Folders: {} | Files: {}",
            config.path.to_str().unwrap(),
            content.folders.len(),
            content.files.len()
        );
        output = string_output_from_files_and_folders(content.folders, content.files);
    }
    for line in output {
        println!("{}", line);
    }

    Ok(())
}
