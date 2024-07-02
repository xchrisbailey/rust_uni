use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Lines, Result},
    path::{Path, PathBuf},
};

use regex::Regex;

fn main() -> Result<()> {
    let mut args = env::args();
    let binding = args.nth(1).expect("Expected a path argument");
    let pattern: &str = &args.next().expect("Expected a pattern argument");
    let path = Path::new(&binding);
    let files = collect_files(path)?;

    for file in files {
        let f = File::open(&file).expect("Could not open file");
        let buf = BufReader::new(f).lines();
        let reg = Regex::new(pattern).expect("Invalid pattern");
        let matches = match_pattern(buf, reg)?;
        if !matches.is_empty() {
            println!("File: {:?}", file);
            for line in matches {
                println!("{}", line);
            }
            println!("\n")
        }
    }

    Ok(())
}

fn match_pattern(content: Lines<BufReader<File>>, pattern: Regex) -> Result<Vec<String>> {
    let mut matches = vec![];
    for (line_num, line) in content.enumerate() {
        let line_str = line.expect("Could not read line");
        if pattern.is_match(&line_str) {
            matches.push(format!("{} - {}", line_num, line_str.trim()));
        }
    }
    Ok(matches)
}

fn collect_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut all_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let sub_files = collect_files(&path)?;
            all_files.extend(sub_files);
        } else {
            all_files.push(path.to_path_buf());
        }
    }
    Ok(all_files)
}
