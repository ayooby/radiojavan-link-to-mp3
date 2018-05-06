use std::env;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::{Class};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    get_links(file_path);
}

fn get_links(file_path: &str) {
    println!("In file {}", file_path);

    let mut links_file = File::create("mp3_files.txt").expect("Error on creating file");
    let f = File::open(file_path).expect("file not found");
    let file = BufReader::new(&f);

    for line in file.lines() {
        let url = line.expect("Unable to read line");
        println!("Getting music from: {}", &url);

        let resp = reqwest::get(&url).unwrap();
        assert!(resp.status().is_success());

        let document = Document::from_read(resp).unwrap();

        let url = document.find(Class("mp3_download_link")).next().unwrap();
        let url_txt = format!("{} \n", url.attr("link").unwrap());

        links_file.write_all(url_txt.as_bytes()).expect("Error on writing file");
    }

    println!("All Done!")
}

