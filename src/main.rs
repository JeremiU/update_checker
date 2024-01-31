mod util;

use std::{env, fs};
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use tokio::time;
use crate::util::*;

const FILE_PREFIX: &str = "page";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    //for more generic use cases, you can run the program to check a different URL or run at a different interval
    let mut url = "https://policy.cornell.edu/policy-library/interim-expressive-activity-policy".to_owned();
    let mut interval_sec: u64 = 30;

    match args.len() {
        2 | 3 => {
            if args[1].starts_with("http") {
                url = args[1].to_owned();
            } else {
                url = format!("https://{}", args[1].to_owned());
            }
            if args.len() == 3 {
                match args[2].parse::<u64>() {
                    Ok(i) => {
                        interval_sec = i;
                    }
                    Err(_) => {
                        println!("Incorrect argument! Must be integer from 30 - 86,400. Set to 30.");
                        interval_sec = 30;
                    }
                }
            }
        }
        0 | 1 => {}
        _ => {
            println!("Running default configuration!");
            println!("Syntax: {} [website url] [interval]", args[0]);
        }
    }

    let mut interval = time::interval(Duration::from_secs(interval_sec));
    let content = html(&url).await.unwrap();

    loop {
        interval.tick().await;

        if !is_current(&content).await.unwrap() {
            let mut file = File::create(file_name(AccessType::WRITE)).unwrap();
            file.write_all(content.as_bytes()).unwrap();
            println!("Site has been updated!");
        } else {
            println!("Local copy is to up to date! ({})", time().as_str());
        }
    }
}

/// Gathers the most recent filename under the "page {iter} {Date}.html" format, where
/// iter - the number iteration from the original
/// date - date when the iteration was pulled from the internet
fn file_name(access: AccessType) -> String {
    let mut iteration = 0;

    let filtered_files: Vec<_> = fs::read_dir("./").unwrap().filter_map(|entry| entry.ok().filter(|e| e.file_name().to_string_lossy().contains(FILE_PREFIX))).collect();

    //find latest file
    for k in filtered_files {
        let str = k.file_name();
        let str2 = str.to_str().unwrap().replace(FILE_PREFIX,"");

        //get the number before the --, i.e iter
        if !str2.contains("--") {continue;}
        let str3 = &str2[1..str2.find("--").unwrap()];
        if str3.parse::<i32>().is_ok() {
            let k = str3.parse::<i32>().unwrap();
            if k > iteration {
                iteration = k;
            }
        }
    }

    if access == AccessType::READ && iteration != 0 {
        //grabs the first file which contains "{FILE_PREFIX} {iteration}" in its name
        let filtered_files: Vec<_> = fs::read_dir("./").unwrap().filter_map(|entry| entry.ok().filter(|e| e.file_name().to_string_lossy().contains(format!("{FILE_PREFIX} {iteration}").as_str()))).collect();
        filtered_files[0].file_name().to_str().unwrap().to_string()
    } else { //called when AccessType::WRITE or when no file exists
        //otherwise, create a new file, with an incremented iteration label
        let time_formatted = time();
        iteration += 1;
        format!("{FILE_PREFIX} {iteration}--{time_formatted}.html")
    }
}

/// Returns whether the website (passed through output) matches the contents of the most recent file
async fn is_current(output: &str) -> Result<bool, std::io::Error> {
    //if the locally saved file doesn't exist, our system is out of date
    let exists = if let Ok(metadata) = fs::metadata(&file_name(AccessType::READ)) {
        metadata.is_file()
    } else {
        false
    };

    if !exists {
        return Ok(false);
    }

    //reads the latest file, and returns whether it is up to date
    let file_name = file_name(AccessType::READ);
    println!("Comparing website with local copy: {file_name}"); //log
    let content = fs::read_to_string(file_name)?;
    Ok(content.trim() == output.trim())
}