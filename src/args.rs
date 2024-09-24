use clap::Parser;

/// A simple CLI tool to output common .gitignore files
#[derive(Parser, Debug)]
#[command(name = "gig")]
pub struct Args {
    /// One or more terms
    #[arg(required = true)]
    pub terms: Vec<String>,
}
