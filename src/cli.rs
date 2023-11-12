use anyhow::Result;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
#[command(override_usage(
    "wc [OPTION]... [FILE]...
  or:  wc [OPTION]... --files0-from=F
Print newline, word, and byte counts for each FILE, and a total line if
more than one FILE is specified.  A word is a non-zero-length sequence of
characters delimited by white space.

With no FILE, or when FILE is -, read standard input.

The options below may be used to select which counts are printed, always in
the following order: newline, word, character, byte, maximum line length."
))]
pub struct Cli {
    file: Vec<String>,

    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// print the character counts
    #[arg(short = 'm', long)]
    chars: bool,

    /// print the newline counts
    #[arg(short, long)]
    lines: bool,

    /// read input from the files specified by
    ///   NUL-terminated names in file F;
    ///   If F is - then read names from standard input
    #[arg(long = "files0-from", id = "F", verbatim_doc_comment)]
    files_from: Option<String>,

    /// print the maximum display width
    #[arg(short = 'L', long)]
    max_line_length: bool,

    /// print the word counts
    #[arg(short, long)]
    words: bool,
}

impl Cli {
    fn is_files_from(&self) -> bool {
        self.files_from.is_some()
    }

    pub fn get_filenames(&self) -> Result<Vec<String>> {
        let mut result = vec![];

        if self.is_files_from() {
            let files_from = self.files_from.clone().unwrap();

            if files_from == "-" {
                todo!()
            } else {
                let filenames = fs::read_to_string(files_from)?;

                for filename in filenames.split('\0') {
                    result.push(filename.to_string());
                }
            }
        } else {
            if self.file.is_empty() {
                result.push("-".to_string());

                return Ok(result);
            }

            for filename in &self.file {
                result.push(filename.to_string());
            }
        }

        Ok(result)
    }

    pub fn get_flags(&self) -> Flags {
        if !self.bytes && !self.chars && !self.lines && !self.max_line_length && !self.words {
            return Default::default();
        }

        Flags {
            bytes: self.bytes,
            chars: self.chars,
            lines: self.lines,
            max_line_length: self.max_line_length,
            words: self.words,
        }
    }
}

pub struct Flags {
    pub bytes: bool,
    pub chars: bool,
    pub lines: bool,
    pub max_line_length: bool,
    pub words: bool,
}
impl Flags {
    pub fn is_single(&self) -> bool {
        let sum = self.bytes as u8
            + self.chars as u8
            + self.lines as u8
            + self.max_line_length as u8
            + self.words as u8;

        sum == 1
    }
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            bytes: true,
            chars: false,
            lines: true,
            max_line_length: false,
            words: true,
        }
    }
}
