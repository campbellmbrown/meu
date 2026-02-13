use anyhow::Result;
use glob::glob;
use regex::Regex;
use std::{fs, path::Path};

pub enum ScanTarget {
    Dir { dir: String, file_glob: String },
    File { path: String },
}

pub struct ScanArgs {
    pub target: ScanTarget,
    pub pattern: String,
    pub stop_on_first_match: bool,
}

pub fn run(args: &ScanArgs) -> Result<()> {
    let re = Regex::new(&args.pattern)?;
    match &args.target {
        ScanTarget::Dir { dir, file_glob } => {
            let search_pattern = format!("{dir}/{file_glob}");
            for entry in glob(&search_pattern)? {
                let path = entry?;
                if path.is_file() && process_file(&path, &re, args.stop_on_first_match)? {
                    return Ok(());
                }
            }
        }
        ScanTarget::File { path } => {
            let path = Path::new(path);
            if !path.is_file() {
                anyhow::bail!("{} is not a valid file", path.display());
            }
            process_file(path, &re, args.stop_on_first_match)?;
        }
    }
    Ok(())
}

fn process_file(path: &Path, re: &Regex, stop_on_first_match: bool) -> Result<bool> {
    let content = fs::read_to_string(path)?;
    for (i, line) in content.lines().enumerate() {
        if re.is_match(line) {
            println!("{}:{}: {}", path.display(), i + 1, line);
            if stop_on_first_match {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
