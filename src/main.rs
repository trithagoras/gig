use std::{collections::HashMap, env, fs::{self, OpenOptions}, io::{self, Write}, path::Path};

use args::Args;
use clap::Parser;
use once_cell::sync::Lazy;
use simpline::SimpLineReader;
use walkdir::{DirEntry, WalkDir};


mod args;
mod simpline;

static KNOWN_ALIASES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("vscode", "visualstudiocode");
    map.insert("c#", "csharp");
    map
});

fn main() -> rustyline::Result<()> {
    let args = Args::parse();
    let paths = get_all_files();

    if args.terms.is_empty() {
        // interactive mode
        let mut filenames = get_filenames(&paths);
        let keys: Vec<String> = KNOWN_ALIASES.keys().map(|&k| k.to_string()).collect();
        filenames.extend(keys); // this concat means it's possible to search for aliases but also means hinting will show aliases as well as filenames
        let slr = SimpLineReader::new("gig> ".into(), filenames);
        let terms = slr.read_words()?;

        let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("./.gitignore")?;

        process_terms(terms, &paths, file)?;
        println!("Wrote to .gitignore");
    } else {
        process_terms(args.terms, &paths, io::stdout())?;
    }

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

fn get_filenames(paths: &Vec<DirEntry>) -> Vec<String> {
    paths.iter().map(|entry| {
        let s = entry.path().file_name().unwrap().to_string_lossy().to_string().to_lowercase();
        s[..s.len() - 10].to_string()
    }).collect()
}

fn process_terms<W: Write>(terms: Vec<String>, files: &Vec<DirEntry>, mut output: W) -> io::Result<()> {
    let mut matched_files = Vec::new();

    for entry in files {
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
            output.write_all(content.as_bytes())?;
            output.write_all(b"\n")?;
        }
    }

    Ok(())
}
