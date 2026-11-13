//! Renders `link-features.h` from `link-features.h.in` with the values
//! configure would fill in.

use std::path::Path;
use std::{fs, io};

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

/// The values configure fills into `link-features.h.in`, the template
/// of the header that `link-includes.h` includes as
/// `<link-grammar/link-features.h>`.
pub struct LinkFeatures<'a> {
    template: String,
    version: Version,
    host_os: &'a str,
    defs: &'a str,
}

impl<'a> LinkFeatures<'a> {
    /// Reads the template from the link-grammar sources at
    /// `source_root`, and the version from its `configure.ac`.
    /// `host_os` and `defs` only show up in the string
    /// `linkgrammar_get_configuration()` returns.
    pub fn new(
        source_root: &Path,
        host_os: &'a str,
        defs: &'a str,
    ) -> io::Result<Self> {
        let template = fs::read_to_string(
            source_root.join("link-grammar/link-features.h.in"),
        )?;
        let configure_ac =
            fs::read_to_string(source_root.join("configure.ac"))?;
        Ok(Self {
            template,
            version: Version::from_configure_ac(&configure_ac),
            host_os,
            defs,
        })
    }

    /// Writes the header to `include_dir/link-grammar/link-features.h`.
    pub fn write_to(&self, include_dir: &Path) -> io::Result<()> {
        let dir = include_dir.join("link-grammar");
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("link-features.h"), self.header())
    }

    /// The template with every `@NAME@` placeholder replaced by its
    /// value.
    fn header(&self) -> String {
        let header = self.values().fold(
            self.template.clone(),
            |header, (name, value)| {
                header.replace(&format!("@{name}@"), &value)
            },
        );
        assert!(
            !header.contains('@'),
            "link-features.h.in has a placeholder that LinkFeatures \
             doesn't fill in"
        );
        header
    }

    /// Each placeholder name with its value.
    fn values(&self) -> impl Iterator<Item = (&str, String)> {
        let Version {
            major,
            minor,
            micro,
        } = self.version;
        FIXED_VALUES
            .map(|(name, value)| (name, value.to_owned()))
            .into_iter()
            .chain([
                ("LINK_MAJOR_VERSION", major.to_string()),
                ("LINK_MINOR_VERSION", minor.to_string()),
                ("LINK_MICRO_VERSION", micro.to_string()),
                ("HOST_OS", self.host_os.to_owned()),
                ("LG_DEFS", self.defs.to_owned()),
            ])
    }
}

/// A link-grammar release number.
struct Version {
    major: u32,
    minor: u32,
    micro: u32,
}

impl Version {
    /// The release number `configure.ac` assigns as
    /// `LINK_MAJOR_VERSION=5` and so on.
    fn from_configure_ac(configure_ac: &str) -> Self {
        let number = |name: &str| {
            configure_ac
                .lines()
                .find_map(|line| {
                    line.strip_prefix(name)?
                        .strip_prefix('=')?
                        .parse()
                        .ok()
                })
                .expect("configure.ac assigns each version number")
        };
        Self {
            major: number("LINK_MAJOR_VERSION"),
            minor: number("LINK_MINOR_VERSION"),
            micro: number("LINK_MICRO_VERSION"),
        }
    }
}
