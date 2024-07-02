use std::{
    env, fs,
    io::Result,
    path::{Path, PathBuf},
};

fn main() -> Result<()> {
    let mut args = env::args();
    let binding = args.nth(1).expect("Expected a path argument");
    let path = Path::new(&binding);
    let files = collect_files(path)?;

    for file in files {
        println!("{}", file.display());
    }

    Ok(())
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
