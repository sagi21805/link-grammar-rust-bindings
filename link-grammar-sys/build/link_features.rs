//! Renders `link-features.h` from `link-features.h.in` with the values
//! configure would fill in.

use std::path::Path;
use std::{fs, io};

/// The placeholders configure fills in with the release number, which
/// `configure.ac` assigns as `NAME=value`.
const VERSION_NAMES: [&str; 3] = [
    "LINK_MAJOR_VERSION",
    "LINK_MINOR_VERSION",
    "LINK_MICRO_VERSION",
];

/// The placeholders whose values are the same in every build.
const FIXED_VALUES: [(&str, &str); 7] = [
    (
        "DEFAULT_VISIBILITY",
        r#"__attribute__((__visibility__("default")))"#,
    ),
    ("CPPFLAGS", ""),
    ("CFLAGS", ""),
    (
        "DISCUSSION_GROUP",
        "https://groups.google.com/d/forum/link-grammar",
    ),
    ("OVERVIEW", "https://en.wikipedia.org/wiki/Link_grammar"),
    (
        "PACKAGE_BUGREPORT",
        "https://github.com/opencog/link-grammar",
    ),
    (
        "PACKAGE_URL",
        "https://opencog.github.io/link-grammar-website",
    ),
];

/// The header that `link-includes.h` includes as
/// `<link-grammar/link-features.h>`.
pub struct LinkFeatures {
    header: String,
}

impl LinkFeatures {
    /// Fills in the template from the link-grammar sources at
    /// `source_root`, with the version from its `configure.ac`.
    /// `host_os` and `defs` only show up in the string
    /// `linkgrammar_get_configuration()` returns.
    pub fn new(
        source_root: &Path,
        host_os: &str,
        defs: &str,
    ) -> io::Result<Self> {
        let template = fs::read_to_string(
            source_root.join("link-grammar/link-features.h.in"),
        )?;
        let configure_ac =
            fs::read_to_string(source_root.join("configure.ac"))?;
        let version = configure_ac
            .lines()
            .filter_map(|line| line.split_once('='))
            .filter(|(name, _)| VERSION_NAMES.contains(name));
        let header = FIXED_VALUES
            .into_iter()
            .chain(version)
            .chain([("HOST_OS", host_os), ("LG_DEFS", defs)])
            .fold(template, |header, (name, value)| {
                header.replace(&format!("@{name}@"), value)
            });
        assert!(
            !header.contains('@'),
            "link-features.h.in has a placeholder that LinkFeatures \
             doesn't fill in"
        );
        Ok(Self { header })
    }

    /// Writes the header to `include_dir/link-grammar/link-features.h`.
    pub fn write_to(&self, include_dir: &Path) -> io::Result<()> {
        let dir = include_dir.join("link-grammar");
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("link-features.h"), &self.header)
    }
}
