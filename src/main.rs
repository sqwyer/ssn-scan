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
    let mut total_possible: usize = 0;
    println!("---------------------------------------------------------");
    for file in WalkDir::new(&args.dir).into_iter().filter_map(|file| file.ok()) {
        let path = &file.path();
        if metadata(path).unwrap().is_file() {
            let result = lib::scan_file(&path);
            if result.is_none() {
                continue;
            }
            match result {
                Some(lines) => {
                    println!("------ Start File {:?} ----->>>", path.as_os_str());
                    let mut l_possible: usize = 0;
                    let mut line_num = 0;
                    for line in lines.iter() {
                        line_num+=1;
                        l_possible+=line.1.numbers.len();
                        println!("File {:?} has {:?} possible SSN(s) on line {}", path.as_os_str(),line.0,line_num);
                    }
                    total_possible += l_possible;
                    total_files += 1;
                    if l_possible > 0 {
                        println!("File {:?} has no possible SSNs", path.as_os_str());
                    }
                },
                None => continue,
            }
        }
    }
    println!("---------------------------------------------------------\nFound possible {} SSN(s) accross {} file(s).", total_possible, total_files);
    Ok(())
}