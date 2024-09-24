use std::{collections::HashMap, env, fs, io::{self, Write}, path::Path};

use args::Args;
use clap::Parser;
use once_cell::sync::Lazy;
use walkdir::{DirEntry, WalkDir};


mod args;

static KNOWN_ALIASES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("vscode", "visualstudiocode");
    map.insert("c#", "csharp");
    map
});

fn main() -> io::Result<()> {
    let args = Args::parse();
    process_terms(args.terms)?;

    Ok(())
}

fn get_all_files() -> Vec<DirEntry> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let gitignore_dir = Path::new(&manifest_dir).join("gitignore").join("templates");
    let mut paths = Vec::new();

    for entry in WalkDir::new(gitignore_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file()) {
        if let Some(file_name) = entry.path().file_name() {
            let file_name = file_name.to_string_lossy().to_lowercase();
            if file_name.ends_with(".gitignore") {
                paths.push(entry);
            }
        }
    }
    paths

}

fn process_terms(terms: Vec<String>) -> io::Result<()> {
    let mut matched_files = Vec::new();

    let paths = get_all_files();
    for entry in &paths {
        for term in &terms {
            let term = match KNOWN_ALIASES.get(&term[..]) {
                Some(term) => *term,
                None => term,
            };
            let fmt_term = format!("{}.gitignore", term.to_lowercase());
            let file_name = entry.file_name().to_string_lossy().to_string().to_lowercase();
            if file_name == fmt_term {
                matched_files.push(entry.path().to_path_buf());
            }
        }
    }

    if matched_files.is_empty() {
        eprintln!("No matching .gitignore files found.");
    } else {
        for file_path in matched_files {
            let content = fs::read_to_string(&file_path)?;
            io::stdout().write_all(content.as_bytes())?;
            io::stdout().write_all(b"\n")?;
        }
    }

    Ok(())
}
