mod lib;

extern crate walkdir;
use quicli::prelude::CliResult;
use walkdir::WalkDir;
use std::{fs::{metadata}};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    dir: String,
}

fn main() -> CliResult  {
    let args = Cli::from_args();
    let mut total_files: u128 = 0;
    let mut total_possible: u128 = 0;
    println!("---------------------------------------------------------");
    for file in WalkDir::new(&args.dir).into_iter().filter_map(|file| file.ok()) {
        let path = &file.path();
        if metadata(path).unwrap().is_file() {
            let buf = lib::read_file(&path);
            let s = match std::str::from_utf8(&buf) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let mut line_num: u128 = 0;
            let mut total_cap: u128 = 0;
            println!("------ Start File {:?} ----->>>", path.as_os_str());
            total_files+=1;
            for line in s.lines() {
                line_num += 1;
                let line_res = lib::scan_text(line);
                let matches = line_res.get(&1);
                if matches.is_none() {
                    continue;
                }
                if line_res.get(&1).unwrap().count > 0 {
                    println!("File {:?} has {:?} possible SSN(s) on line {}", path.as_os_str(), line_res.get(&1).unwrap().count, line_num);
                }
                total_cap += line_res.get(&1).unwrap().count;

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