// lf
use std::env;
use std::error::Error;
use std::fs::DirEntry;
use std::process::exit;

extern crate termcolor;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub mod error;
pub mod functions;
pub mod sort;
use functions::*;
use sort::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Set colored output
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;

    let args: Vec<String> = env::args().skip(1).collect();
    let config = get_config_from_args(args)?;
    let content = get_folders_files(&config);

    match run(config, content, &mut stdout) {
        Err(e) => {
            eprintln!("RUN ERROR: {}", e);
            exit(1);
        }
        Ok(()) => exit(0),
    };
}

fn run(config: Config, mut content: Content, stdout: &mut StandardStream) -> Result<(), Box<dyn Error>> {
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
    let files: Vec<DirEntry>;

    print!("{:<19}", env::current_dir().unwrap().display());
    if config.file_filter {
        println!(
            "{} *.{} | Files: {}",
            config.path.to_str().unwrap(),
            config.file_type,
            content.files.len()
        );
        files = get_file_from_ending(content.files, &config.file_type);
        let file_output = string_output_from_files(files);

        // printing
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        for line in file_output {
            println!("{}", line);
        }
    } else {
        println!(
            "{} Folders: {} | Files: {}",
            config.path.to_str().unwrap(),
            content.folders.len(),
            content.files.len()
        );
        let (folder_output, file_output) =
            string_output_from_files_and_folders(content.folders, content.files);
        // printing
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        for line in folder_output {
            println!("{}", line);
        }
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        for line in file_output {
            println!("{}", line);
        }
    }

    Ok(())
}
