#[cfg(not(test))]
use std::process::Command;

#[cfg(not(test))]
use std::fs::File;

#[cfg(not(test))]
use std::io::Write;

#[cfg(not(test))]
fn main() {
    seek_the_path();
    walk_the_path();
}

macro_rules! koan {
    ($name:expr) => (
        include!(concat!("koans/", $name, ".rs"));
    );
}

#[cfg(not(test))]
fn seek_the_path() {
    let koans = vec!["the_truth", "addition"];
    let mut path = File::create("src/path_to_enlightenment.rs").unwrap();

    for koan in koans {
        write!(&mut path, "koan!(\"{}\");\n", koan).unwrap();
    }
}

#[cfg(not(test))]
fn walk_the_path() {
    let success = Command::new("cargo")
        .arg("test")
        .arg("-q")
        .status()
        .unwrap()
        .success();
    println!("Enlightenment: {}", success);
}

#[cfg(test)]
mod path_to_enlightenment;
