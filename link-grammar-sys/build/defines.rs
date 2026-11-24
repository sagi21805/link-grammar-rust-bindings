//! The defines configure finds on each target the bundled build
//! supports. `main.rs` compiles link-grammar with them, and `cargo xtask
//! regen` gives them to bindgen, so the private struct layouts it
//! generates match the compiled library.

/// A target whose configure results the bundled build knows.
pub enum Target {
    LinuxGnu,
    MacOs,
}

impl Target {
    /// The defines configure finds on this target, with the optional
    /// libraries turned off.
    pub fn defines(
        &self,
    ) -> impl Iterator<Item = &'static (&'static str, &'static str)> {
        let own = match self {
            Self::LinuxGnu => LINUX_GNU_DEFINES,
            Self::MacOs => MACOS_DEFINES,
        };
        UNIX_DEFINES.iter().chain(own)
    }
}

/// What configure finds on every supported target, minus the macros no
/// compiled source reads. Regex comes from libc's `<regex.h>`.
const UNIX_DEFINES: &[(&str, &str)] = &[
    ("_DEFAULT_SOURCE", "1"),
    ("_GNU_SOURCE", "1"),
    ("TLS", "_Thread_local"),
    ("HAVE_REGEX_H", "1"),
    ("HAVE_ALLOCA_H", "1"),
    ("HAVE_ALIGNED_ALLOC", "1"),
    ("HAVE___BUILTIN_TYPES_COMPATIBLE_P", "1"),
    ("HAVE_TYPEOF", "1"),
    ("typeof", "__typeof__"),
    ("HAVE_STRNDUP", "1"),
    ("HAVE_STRTOK_R", "1"),
    ("HAVE_STRERROR_R", "1"),
];

/// What glibc Linux adds, which is `locale_t` in `<locale.h>`, the GNU
/// `strerror_r` that returns a `char *`, and `malloc_trim`.
const LINUX_GNU_DEFINES: &[(&str, &str)] = &[
    ("HAVE_LOCALE_T_IN_LOCALE_H", "1"),
    ("STRERROR_R_CHAR_P", "1"),
    ("HAVE_MALLOC_TRIM", "1"),
];

/// What macOS needs for `locale_t`. configure finds it in `<locale.h>`,
/// but `iswspace_l` and the other `isw*_l` functions are declared only
/// in `<xlocale.h>`, so the sources must include that header.
const MACOS_DEFINES: &[(&str, &str)] =
    &[("HAVE_LOCALE_T_IN_XLOCALE_H", "1"), ("HAVE_XLOCALE_H", "1")];
