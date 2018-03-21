extern crate rustc_demangle;

use std::io::{self, BufRead, Write};
use rustc_demangle::demangle;

fn do_read<I: BufRead, O: Write>(input: I, mut output: O) -> Result<(), io::Error> {
    for line in input.lines() {
        let line = line?;

        if let Some(index) = line.find("_ZN") {
            let s = &line[index..];
            let sym_end = s.find(|c: char| c == ':' || c == '@' || c.is_whitespace()).unwrap_or(s.len());

            write!(
                output,
                "{}{}{}\n",
                &line[..index],
                demangle(&s[..sym_end]),
                &s[sym_end..]
            )?;
        } else {
            write!(output, "{}\n", line)?;
        }
    }

    output.flush()?;

    Ok(())
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (input, output) = (stdin.lock(), stdout.lock());

    do_read(input, output).expect("wrangle failed")
}
