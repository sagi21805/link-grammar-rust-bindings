//! The scratch program `cargo xtask package-check` builds against the
//! packaged `link-grammar-sys`. It runs from a directory with no `data/`,
//! so the dictionary must come from the packaged crate.

use link_grammar_sys::*;

fn main() {
    // SAFETY: Each handle is checked or comes from a successful call, and
    // is deleted once, after everything that uses it.
    unsafe {
        let dictionary = dictionary_create_lang(c"en".as_ptr());
        assert!(!dictionary.is_null(), "the package has no English dictionary");
        let options = parse_options_create();
        let sentence = sentence_create(
            c"The lander touched down on Mars.".as_ptr(),
            dictionary,
        );
        let found = sentence_parse(sentence, options);
        assert_eq!(found, 16, "link-parser finds 16 linkages");
        sentence_delete(sentence);
        parse_options_delete(options);
        dictionary_delete(dictionary);
    }
    println!("The packaged link-grammar-sys parsed a sentence.");
}
