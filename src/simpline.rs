use std::borrow::Cow::{self};

use rustyline::{error::ReadlineError, highlight::Highlighter, hint::Hinter, validate::Validator, CompletionType, Config, Context, EditMode, Editor};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

struct SimpHinter {
    completions: Vec<String>
}

impl Hinter for SimpHinter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        if line.is_empty() || pos < line.len() || line.ends_with(' ') {
            return None;
        }

        // find the start of the current word by looking for the nearest whitespace
        let mut start = pos;
        while start > 0 && !line.as_bytes()[start - 1].is_ascii_whitespace() {
            start -= 1;
        }
        let word = &line[start..pos];

        // find the first completion that hasn't been used and starts with the current word
        let hint = self.completions.iter()
            .filter(|&c| !line.contains(c) && c.starts_with(word))
            .next();

        if let Some(completion) = hint {
            if completion == word {
                return None;
            }
            // return the remaining part of the completion after the current word
            return Some(completion[word.len()..].to_string());
        }

        None
    }
}

struct SimpValidator {
}

impl Validator for SimpValidator {
}

#[derive(Helper, Completer, Hinter, Validator, Highlighter)]
struct SimpHelper {
    #[rustyline(Highlighter)]
    highlighter: SimpHighlighter,
    #[rustyline(Validator)]
    validator: SimpValidator,
    #[rustyline(Hinter)]
    hinter: SimpHinter,
}

struct SimpHighlighter {
}

impl Highlighter for SimpHighlighter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        // apply gray color (ANSI code) to the hint
        format!("\x1b[90m{}\x1b[0m", hint).into()
    }
}

pub struct SimpLineReader {
    completions: Vec<String>,
    prompt: String,
}

impl SimpLineReader {
    pub fn new(prompt: String, completions: Vec<String>) -> Self {
        Self { prompt, completions }
    }

    pub fn read_line(&self) -> rustyline::Result<String> {
        let config = Config::builder()
            .history_ignore_space(true)
            .completion_type(CompletionType::Circular)
            .edit_mode(EditMode::Vi)
            .build();

        let h = SimpHelper {
            highlighter: SimpHighlighter {  },
            hinter: SimpHinter { completions: self.completions.clone() },
            validator: SimpValidator { },
        };
        let mut rl = Editor::with_config(config)?;
        rl.set_helper(Some(h));

        let readline = rl.readline(&self.prompt);

        match readline {
            Ok(line) => {
                Ok(line)
            }
            Err(ReadlineError::Eof) => {
                Err(ReadlineError::Eof)
            }
            Err(ReadlineError::Interrupted) => {
                Err(ReadlineError::Interrupted)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    pub fn read_words(&self) -> rustyline::Result<Vec<String>> {
        let mut v = Vec::new();
        let line = self.read_line()?;
        
        for s in line.split_whitespace() {
            v.push(s.into());
        }

        Ok(v)
    }
}
