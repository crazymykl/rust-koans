use std::fs::{OpenOptions};

fn main() {
    OpenOptions::new().create(true)
        .write(true)
        .open("src/path_to_enlightenment.rs")
        .unwrap();
}
