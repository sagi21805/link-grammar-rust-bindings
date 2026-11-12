//! Prints the best linkage of each line on stdin as a diagram.
//!
//! ```sh
//! echo "The lander touched down on Mars." | cargo run --example parse
//! ```

use std::io::{self, BufRead};

use link_grammar::{Dictionary, Language, ParseOptions, Sentence};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dictionary = Dictionary::new(Language::English)?;
    let options = ParseOptions::new();
    for line in io::stdin().lock().lines() {
        let mut sentence = Sentence::new(&dictionary, &line?)?;
        let found = sentence.parse(&options)?;
        println!("Found {found} linkages");
        if let Some(linkage) = sentence.linkage(0, &options) {
            println!("{}", linkage.diagram(80));
        }
    }
    Ok(())
}
