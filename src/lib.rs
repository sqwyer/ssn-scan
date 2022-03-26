use std::{path::{Path}, fs::File, io::Read, collections::HashMap};

use regex::Regex;


pub const MATCH: &str = r"(\d{9}+)|(\d{3}-\d{2}-\d{4}+)";

pub struct LineResult {
    pub numbers: Vec<String>,
    pub count: u128
}

// pub struct FileResult<'a> {
//     path: &'a Path,
//     lines: &'a Vec<&'a LineResult<'a>>
// }

pub fn read_file(path: &Path) -> Vec<u8> {
    let mut file_content = Vec::new();
    let mut file = File::open(&path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}

pub fn scan_text(text: &str) -> HashMap::<u128,LineResult> {
    let regex_str = Regex::new(MATCH).unwrap();
    let mut all_lines: HashMap<u128, LineResult> = HashMap::new();
    let mut line_num: u128 = 0;
    for line in text.lines() {
        line_num += 1;
        let mut numbers = vec![];
        for cap in regex_str.captures_iter(line) {
            numbers.push(cap.get(0).map_or("".to_string(), |m| m.as_str().to_string()));
        }
        let count = u128::try_from(numbers.len()).unwrap();
        let line_res = LineResult {
            numbers: numbers,
            count: count
        };
        all_lines.insert(line_num, line_res);
    }
    all_lines
}