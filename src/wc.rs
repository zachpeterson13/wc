use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    ops::AddAssign,
};

use anyhow::Result;

use crate::cli;

#[derive(Default, Clone)]
struct Counts {
    name: String,
    bytes: usize,
    chars: usize,
    lines: usize,
    words: usize,
    max_line_length: usize,
}

impl Counts {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn write_counts(&self, flags: &cli::Flags) {
        if flags.lines {
            print!("{} ", self.lines);
        }

        if flags.words {
            print!("{} ", self.words);
        }

        if flags.bytes {
            print!("{} ", self.bytes);
        }

        if flags.chars {
            print!("{} ", self.chars);
        }

        if flags.max_line_length {
            print!("{} ", self.max_line_length);
        }

        println!("{} ", self.name);
    }
}

impl AddAssign for Counts {
    fn add_assign(&mut self, rhs: Self) {
        self.bytes += rhs.bytes;
        self.chars += rhs.chars;
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.max_line_length += rhs.max_line_length;
    }
}

pub fn wc(args: cli::Cli) -> Result<()> {
    let filenames = args.get_filenames();
    let mut totals = Counts::new("total".to_string());
    let flags = args.get_flags();

    for filename in &filenames {
        let counts = wc_file(filename)?;

        counts.write_counts(&flags);

        totals += counts;
    }

    if filenames.len() > 1 {
        totals.write_counts(&flags);
    }

    Ok(())
}

fn wc_file(filename: &str) -> Result<Counts> {
    let mut counts = Counts::new(filename.to_string());

    let file = File::open(filename)?;
    let metadata = file.metadata()?;
    let buf_reader = BufReader::new(file);

    counts.bytes += metadata.len() as usize;

    for line in buf_reader.lines().flatten() {
        counts.lines += 1;
        counts.chars += line.chars().count() + 1;
        counts.words += line.split_whitespace().count();
        counts.max_line_length = cmp::max(counts.max_line_length, line.len());
    }

    Ok(counts)
}

#[cfg(test)]
mod tests {}
