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

    fn write_counts(&self, flags: &cli::Flags, offset: usize) {
        if flags.lines {
            print!("{:>offset$} ", self.lines);
        }

        if flags.words {
            print!("{:>offset$} ", self.words);
        }

        if flags.bytes {
            print!("{:>offset$} ", self.bytes);
        }

        if flags.chars {
            print!("{:>offset$} ", self.chars);
        }

        if flags.max_line_length {
            print!("{:>offset$} ", self.max_line_length);
        }

        println!("{}", self.name);
    }

    fn get_offset(&self) -> usize {
        let mut temp = self.bytes;
        let mut count = 0;

        while temp != 0 {
            temp /= 10;
            count += 1;
        }

        count
    }
}

impl AddAssign<&Self> for Counts {
    fn add_assign(&mut self, rhs: &Self) {
        self.bytes += rhs.bytes;
        self.chars += rhs.chars;
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.max_line_length = cmp::max(self.max_line_length, rhs.max_line_length);
    }
}

impl AddAssign for Counts {
    fn add_assign(&mut self, rhs: Self) {
        self.bytes += rhs.bytes;
        self.chars += rhs.chars;
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.max_line_length = cmp::max(self.max_line_length, rhs.max_line_length);
    }
}

pub fn wc(args: cli::Cli) -> Result<()> {
    let filenames = args.get_filenames();
    let mut totals = Counts::new("total".to_string());
    let flags = args.get_flags();

    let mut counts_vec = vec![];

    for filename in &filenames {
        let counts = wc_file(filename)?;

        totals += &counts;

        counts_vec.push(counts);
    }

    // offset needs to be set if multiple flags are set and multiple files
    let offset = if counts_vec.len() == 1 && flags.is_single() {
        0
    } else {
        totals.get_offset()
    };

    for counts in counts_vec {
        counts.write_counts(&flags, offset);
    }

    if filenames.len() > 1 {
        totals.write_counts(&flags, offset);
    }

    Ok(())
}

fn wc_file(filename: &str) -> Result<Counts> {
    let mut counts = Counts::new(filename.to_string());

    let file = File::open(filename)?;
    // TODO: add some kind of check to make sure `file` is a file and not a directory or something
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines().flatten() {
        counts.lines += 1;
        counts.bytes += line.len() + 1;
        counts.chars += line.chars().count() + 1;
        counts.words += line.split_whitespace().count();
        counts.max_line_length = cmp::max(counts.max_line_length, line.len());
    }

    Ok(counts)
}

#[cfg(test)]
mod tests {}
