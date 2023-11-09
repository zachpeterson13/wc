use clap::Parser;

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
    #[arg(short, long)]
    bytes: bool,

    /// print the character counts
    #[arg(short, long)]
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
