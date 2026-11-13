//! Prints the best linkage of each line on stdin as a diagram.
//!
//! ```sh
//! echo "The lander touched down on Mars." | cargo run --example parse
//! ```

use link_grammar::{Dictionary, Language, Sentence, Verbosity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    link_grammar::set_verbosity(Verbosity::Silent);
    let dictionary = Dictionary::new(Language::English)?;

    let hp = include_str!("Harry-Potter-1.txt");
    for line in hp.lines() {
        if line.is_empty() {
            continue;
        }
        let mut sentence = Sentence::new(&dictionary, line)?;
        let found = sentence.parse();
        match found {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("Cannot parse line: {}", line);
                continue;
            }
        }
        if let Some(linkage) = sentence.linkage(0) {
            println!("{}", linkage.diagram(100000));
        }
    }
    Ok(())
}
