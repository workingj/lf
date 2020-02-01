// Module sort
// Contains all the sort functions

use std::error::Error;
use std::fs::DirEntry;

pub fn sort_size_ascending(items: &mut Vec<DirEntry>) -> Result<(), Box<dyn Error>> {
    Ok(items.sort_by(|a, b| {
        a.metadata()
            .unwrap()
            .len()
            .cmp(&b.metadata().unwrap().len())
    }))
}

pub fn sort_size_descending(items: &mut Vec<DirEntry>) -> Result<(), Box<dyn Error>> {
    Ok(items.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .len()
            .cmp(&a.metadata().unwrap().len())
    }))
}

pub fn sort_name_ascending(items: &mut Vec<DirEntry>) -> Result<(), Box<dyn Error>> {
    Ok(items.sort_by(|a, b| {
        a.file_name()
            .to_str()
            .unwrap()
            .cmp(&b.file_name().to_str().unwrap())
    }))
}

pub fn sort_name_descending(items: &mut Vec<DirEntry>) -> Result<(), Box<dyn Error>> {
    Ok(items.sort_by(|a, b| {
        b.file_name()
            .to_str()
            .unwrap()
            .cmp(&a.file_name().to_str().unwrap())
    }))
}

pub fn sort_time_ascending(items: &mut Vec<DirEntry>) -> Result<(), Box<dyn Error>> {
    Ok(items.sort_by(|a, b| {
        a.metadata()
            .expect("could not read metadata")
            .modified()
            .expect("could not read metadata.modified")
            .elapsed()
            .expect("could not get duration")
            .as_secs()
            .cmp(
                &b.metadata()
                    .expect("could not read metadata")
                    .modified()
                    .expect("could not read metadata.modified")
                    .elapsed()
                    .expect("could not get duration")
                    .as_secs(),
            )
    }))
}

pub fn sort_time_descending(items: &mut Vec<DirEntry>) -> Result<(), Box<dyn Error>> {
    Ok(items.sort_by(|a, b| {
        b.metadata()
            .expect("could not read metadata")
            .modified()
            .expect("could not read metadata.modified")
            .elapsed()
            .expect("could not get duration")
            .as_secs()
            .cmp(
                &a.metadata()
                    .expect("could not read metadata")
                    .modified()
                    .expect("could not read metadata.modified")
                    .elapsed()
                    .expect("could not get duration")
                    .as_secs(),
            )
    }))
}
