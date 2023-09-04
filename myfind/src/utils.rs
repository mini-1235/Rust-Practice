use colored::*;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
pub fn find<P: AsRef<Path>>(
    root: P,
    regex: &Regex,
    verbose: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut res = Vec::new();
    walk_tree(root.as_ref(), regex, &mut res, verbose)?;
    Ok(res)
}

fn walk_tree(
    dir: &Path,
    regex: &Regex,
    res: &mut Vec<String>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            //if verbose print directly
            if verbose {
                println!("{}", path.to_string_lossy().truecolor(0, 255, 0));
            }
            if path.is_dir() {
                walk_tree(&path, regex, res, verbose)?;
            } 
            else{
                if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                    if regex.is_match(filename) {
                        res.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    //if dir is a file, then check if it matches the regex
    else{
        if let Some(filename) = dir.file_name().and_then(|s| s.to_str()) {
            if regex.is_match(filename) {
                res.push(dir.to_string_lossy().to_string());
            }
        }
    }
    Ok(())
}
