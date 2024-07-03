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
    let files = collect_files(Path::new(&binding))?;

    let pattern: &str = &args.next().expect("Expected a pattern argument");

    for file in files {
        let matches = match_pattern(
            BufReader::new(File::open(&file).expect("Could not open file")).lines(),
            Regex::new(pattern).expect("Invalid pattern"),
        )?;

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

    all_files.sort();

    Ok(all_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::create_dir;

    #[test]
    fn test_collect_files_with_empty_directory() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let dir_path = temp_dir.path().join("empty_dir");
        create_dir(&dir_path).expect("Failed to create empty directory");

        let result = collect_files(&dir_path);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<PathBuf>::new());
    }

    #[test]
    fn test_collect_files_with_single_file() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = temp_dir.path().join("file.txt");
        File::create(&file_path).expect("Failed to create file");

        let result = collect_files(temp_dir.path());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![file_path]);
    }

    #[test]
    fn test_collect_files_with_nested_directories() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let dir_path = temp_dir.path().join("nested_dir");
        create_dir(&dir_path).expect("Failed to create nested directory");

        let sub_dir_path = dir_path.join("sub_dir");
        create_dir(&sub_dir_path).expect("Failed to create sub directory");

        let file_path1 = dir_path.join("file1.txt");
        File::create(&file_path1).expect("Failed to create file1");

        let file_path2 = sub_dir_path.join("file2.txt");
        File::create(&file_path2).expect("Failed to create file2");

        let result = collect_files(temp_dir.path());

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![file_path1, file_path2]);
    }
}
