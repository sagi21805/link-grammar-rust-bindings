//! Prints the best linkage of each line on stdin as a diagram.
//!
//! ```sh
//! echo "The lander touched down on Mars." | cargo run --example parse
//! ```

use std::io::{self, BufRead};

use link_grammar::{Dictionary, Language, Sentence, Verbosity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    link_grammar::set_verbosity(Verbosity::Silent);
    let dictionary = Dictionary::new(Language::English)?;
    for line in io::stdin().lock().lines() {
        let mut sentence = Sentence::new(&dictionary, &line?)?;
        let found = sentence.parse()?;
        println!("Found {found} linkages");
        if let Some(linkage) = sentence.linkage(0) {
            println!("{}", linkage.diagram(80));
        }
    }
    Ok(())
}
