#[cfg(not(test))]
use std::process::Command;

#[cfg(not(test))]
use std::fs::{File, OpenOptions};

#[cfg(not(test))]
use std::io::{BufRead, BufReader, Write};

#[cfg(not(test))]
fn main() {
    let message = match walk_the_path() {
        true => {
            match seek_the_path() {
                true => "Eternity lies ahead of us, and behind. Your path is not yet finished.",
                false => "What is the sound of one hand clapping (for you)?",
            }
        }
        false => "Meditate on your approach and return. Mountains are merely mountains.",
    };

    println!("{}", message);
}

macro_rules! koan {
    ($name:expr) => (
        include!(concat!("koans/", $name, ".rs"));
    );
}

#[cfg(not(test))]
fn seek_the_path() -> bool {
    let mut koans = BufReader::new(File::open("src/koans.txt").unwrap()).lines();
    let mut path = OpenOptions::new()
        .read(true)
        .write(true)
        .open("src/path_to_enlightenment.rs")
        .unwrap();
    let passed_count = BufReader::new(&path).lines().count();

    if let Some(Ok(next_koan)) = koans.nth(passed_count) {
        println!("Ahead of you lies {}.", next_koan);
        write!(&mut path, "koan!(\"{}\");\n", next_koan).unwrap();
        true
    } else {
        println!("There will be no more tasks.");
        false
    }
}

#[cfg(not(test))]
fn walk_the_path() -> bool {
    Command::new("cargo")
        .arg("clean")
        .status()
        .unwrap()
        .success();
    Command::new("cargo")
        .arg("test")
        .arg("-q")
        .status()
        .unwrap()
        .success()
}

#[cfg(test)]
mod path_to_enlightenment;
