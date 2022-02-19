use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::process::Command;

pub fn hello() {
    println!("Hello, world! - lib");
}

pub fn get_input(day: u8) -> String {
    let path = format!("input/day{}.txt", day);
    if let Ok(mut input_file) = File::open(&path) {
        let mut input = String::new();
        input_file.read_to_string(&mut input).unwrap();
        input
    } else {
        download_input(day, &path)
    }
}

fn download_input(day: u8, path: &str) -> String {
    let url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let session = format!("session={}", get_session());
    let input = Command::new("curl")
        .arg(url)
        .arg("--cookie")
        .arg(session)
        .arg("--http1.1")
        .output()
        .unwrap()
        .stdout;

    let mut input_file = File::create(path).unwrap();
    input_file.write(&input).unwrap();
    String::from_utf8_lossy(&input).into()
}

fn get_session() -> String {
    let mut session_file = File::open(".aoc_session").unwrap();
    let mut session = String::new();
    session_file
        .read_to_string(&mut session)
        .expect("Unable to read .aoc_session");
    session
}
