extern crate walkdir;
use quicli::prelude::CliResult;
use regex::Regex;
use walkdir::WalkDir;
use std::{fs::{metadata, File}, path::Path, io::Read};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    dir: String,
}

// struct LineResult {
//     // numbers: Vec<i32>,
//     numbers_int: i128
// }

fn read_file(path: &Path) -> Vec<u8> {
    let mut file_content = Vec::new();
    let mut file = File::open(&path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}

fn main() -> CliResult  {
    let args = Cli::from_args();
    let reg = Regex::new(r"\d{9}+").unwrap();
    let reg2 = Regex::new(r"\d{3}-\d{2}-\d{4}+").unwrap();
    let mut total_files: i128 = 0;
    let mut total_possible: i128 = 0;
    println!("---------------------------------------------------------");
    for file in WalkDir::new(&args.dir).into_iter().filter_map(|file| file.ok()) {
        let path = &file.path();
        if metadata(path).unwrap().is_file() {
            let buf = read_file(&path);
            let s = match std::str::from_utf8(&buf) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let mut line_num: i128 = 0;
            let mut total_cap: i128 = 0;
            println!("------ Start File {:?} ----->>>", path.as_os_str());
            total_files+=1;
            for line in s.lines() {
                let mut line_possible: i128 = 0;
                line_num+=1;
                // let mut captures = vec![];
                for _ in reg.captures_iter(line) {
                    // captures.push(cap.get(0).map_or("", |m| m.as_str()).parse::<i32>().unwrap());
                    line_possible+=1;
                }
                for _ in reg2.captures_iter(line) {
                    // captures.push(cap.get(0).map_or("", |m| m.as_str()).parse::<i32>().unwrap());
                    line_possible+=1;
                }
                // let line_result = LineResult {
                //     numbers_int: line_possible
                // };
                if line_possible > 0 {
                    println!("File {:?} has {:?} possible SSN(s) on line {}", path.as_os_str(), line_possible, line_num);
                }
                total_cap += line_possible;

            }
            if total_cap == 0 {
                println!("This file has no possible SSNs");
            }
            total_possible += total_cap;
        }
    }
    println!("---------------------------------------------------------\nFound possible {} SSN(s) accross {} file(s).", total_possible, total_files);
    Ok(())
}