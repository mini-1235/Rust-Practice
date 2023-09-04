use colored::*;
use regex::Regex;
use std::env;
use std::process;
use tracing::{info, error,Level};
use tracing_subscriber;
mod utils;
fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    info!("starting up");
    let args: Vec<String> = env::args().collect();

    // if argument len is not 3 or 4 , then print error message and exit
    //support more than 1 regex expression with pattern count specified -p<#>
    if args.len() < 4 {
        println!("Usage: find [-v] <path> [-p<#>] <regex>");
        println!("For example: find -v . -p2 Screen shot");
        error!("invalid argument length");
        process::exit(1);
    }
    let verbose = args[1] == "-v" || args[1] == "--verbose";
    let pattern_idx = if verbose { 4 } else { 3 };
    
    //pattern  starts from idx+1 to the end
    let pattern_count = args[pattern_idx - 1]
        .chars()
        .nth(2)
        .unwrap()
        .to_digit(10)
        .unwrap();
    let pattern_count = pattern_count as usize;
    if pattern_count != args.len()-pattern_idx {
        println!("pattern count does not match the number of patterns");
        error!("pattern count does not match the number of patterns");
        process::exit(1);
    }
    println!("pattern count: {}", pattern_count);
    let pattern = &args[pattern_idx..];
    //debug pattern
    // for i in 0..pattern.len(){
    //     println!("pattern {}: {}",i,pattern[i]);
    // }
    let mut total_match = 0;
    let mut files = Vec::new();
    for i in 0..pattern.len() {
        println!("Searching for pattern: {}", pattern[i].truecolor(255, 0, 0));
        let regex = pattern[i].to_string();
        let regex = match Regex::new(&regex) {
            Ok(regex) => regex,
            Err(err) => {
                println!("Invalid regex expression '{}':{}", regex, err);
                error!("Invalid regex expression '{}':{}", regex, err);
                process::exit(1);
            }
        };
        if verbose {
            match utils::find(&args[2], &regex, true) {
                Ok(matches) => {
                    if matches.is_empty() {
                        // println!("None matched");
                        total_match += 0;
                    } else {
                        // println!("find matches: {}", matches.len());
                        total_match += matches.len();
                        for file in matches {
                            // println!("{}", file);
                            files.push(file);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("error:{}", error);
                    error!("error:{}", error);
                    process::exit(1);
                }
            }
        } else {
            match utils::find(&args[1], &regex, false) {
                Ok(matches) => {
                    if matches.is_empty() {
                        // println!("None matched");
                        total_match += 0;
                    } else {
                        // println!("find matches: {}", matches.len());
                        total_match += matches.len();
                        for file in matches {
                            // println!("{}", file);

                            files.push(file);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("error:{}", error);
                    error!("error:{}", error);
                    process::exit(1);
                }
            }
        }
    }
    if total_match == 0 {
        println!("None matched");
    } else {
        //sort the files by alphabetical order
        files.sort();
        //remove duplicates
        files.dedup();
        //update total match 
        total_match = files.len();
        println!("find matches: {}", total_match);
        for file in files {
            println!("{}", file);
        }
    }
}
