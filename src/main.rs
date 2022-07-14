
use walkdir::WalkDir;
use std::{fs, env, fmt::Write as _};
use rayon::prelude::*;
use prettytable::Table;
#[macro_use] extern crate prettytable;

#[derive(Clone)]
pub struct File {
    pub path: String,
    pub hash: String
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("\nPath not provided, analyzing current directory...\n");
        args.push(String::from("."));
    }
    let path = &args[1];
    println!("\nAnalyzing {}...\n", path);
    let file_list = get_files_recursive(path);
    let hashed_files: Vec<File> = file_list.par_iter().map(|file| get_hash_file(file)).collect();
    output_duplicate_files(get_duplicates(hashed_files));
}

fn get_duplicates(hashed_files: Vec<File>) -> Vec<File> {
    let mut duplicates: Vec<File> = Vec::new();
    for file in &hashed_files {
        if hashed_files.par_iter().filter(|f| f.hash == file.hash).count() > 1 {
            duplicates.push(file.clone());
        }
    }
    duplicates
}

fn get_files_recursive(base_path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    for file in WalkDir::new(base_path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            files.push(file.path().to_string_lossy().to_string());
        }
    }
    files
}

fn output_duplicate_files(mut duplicates: Vec<File>) {
    if duplicates.is_empty() {
        println!("No duplicates found");
        return;
    }
    duplicates.sort_by(|a, b| a.hash.cmp(&b.hash));
    let mut table = Table::new();
    table.add_row(row!["FILE PATH", "FILE HASH MD5"]);

    println!();
    let mut last_hash_row = String::new();
    for entry in &duplicates {
        if entry.hash != last_hash_row && !last_hash_row.is_empty(){
            table.add_row(row!["", ""]);
        }
        table.add_row(row![entry.path, entry.hash]);
        last_hash_row = entry.hash.clone();
    }
    table.printstd();
    println!("\nDuplicates found: {}\n", duplicates.len());
}

fn bytes_to_string(bytes: &[u8; 16]) -> String {
    let mut string = String::new();
    for byte in bytes {
        let _ = write!(string, "{:x}", byte);
    }
    string
}

fn get_hash_file(path: &str) -> File {
    let bytes = fs::read(path).unwrap();
    File { path: path.to_string(), hash: bytes_to_string(&md5::compute(&bytes).0)}
}
