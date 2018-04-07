#[macro_use]
extern crate structopt;
use structopt::StructOpt;

extern crate chrono;
use chrono::{DateTime, Local};


use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;
use std::process;
use std::time::SystemTime;
// use std::os::unix::fs::PermissionsExt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Output file
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    // if let Err(ref e) = run(Path::new(".")) {
    if let Err(ref e) = run(&opt.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(dir: &Path) -> Result<(), Box<Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                    .file_name()
                    .into_string()
                    .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            // check if directory
            let mut dir = String::new();
            if size == 0 {dir = "D".to_string();}
            else {dir = "F".to_string();}

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

            println!(
                    "{}{:>11}  {}  {}",
                    dir,
                    bytes,
                    modified.format("%_d. %b %H:%M").to_string(),
                    file_name,
            );
        }
    }
    Ok(())
}