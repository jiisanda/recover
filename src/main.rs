use anyhow::Result;
use walkdir::{DirEntry, WalkDir};
use std::path::Path;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let files = scan_directory(&cli.directory, &cli.exclude)?;

    println!("Found files (excluding {:?}):", cli.exclude);
    for file in files {
        println!("    {}", file);
    }

    Ok(())
}

pub fn scan_directory<P: AsRef<Path>>(path: P, exclude_patterns: &[String]) -> Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if should_exclude(&entry, exclude_patterns) {
            if entry.file_type().is_dir() {
                continue;
            }
            continue;
        }

        if entry.file_type().is_file() {
            files.push(entry.path().display().to_string());
        }
    }

    Ok(files)
}

fn should_exclude(entry: &DirEntry, patterns: &[String]) -> bool {
    let path = entry.path().to_string_lossy();

    for pattern in patterns {
        if !pattern.contains('*') && path.contains(pattern) {
            return true;
        }

        // handle file patterns like *.log
        if pattern.starts_with('*') {
            let extension = pattern.trim_start_matches("*.");
            if let Some(file_ext) = entry.path().extension() {
                if file_ext == extension {
                    return true;
                }
            }
        }
    }
    false
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    // Directory to scan
    #[arg(short, long)]
    directory: String,

    // patterns to excludes (comma-separated)
    // Ex: "*.tmp,*.log,target"
    #[arg(short, long, value_delimiter = ',', default_value = "")]
    exclude: Vec<String>,
}
