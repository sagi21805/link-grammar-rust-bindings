/// `as_str`, `Display` and comparisons with strings for types that wrap a
/// string borrowed from a linkage, given as `Type.field`.
macro_rules! borrowed_str {
    ($($ty:ident . $field:tt),*) => {$(
        impl<'l> $ty<'l> {
            #[doc = concat!("The ", stringify!($ty), " as link-grammar prints it.")]
            pub fn as_str(self) -> &'l str { self.$field }
        }

        impl ::std::fmt::Display for $ty<'_> {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                f.write_str(self.$field)
            }
        }

        impl PartialEq<str> for $ty<'_> {
            fn eq(&self, other: &str) -> bool { self.$field == other }
        }

        impl PartialEq<&str> for $ty<'_> {
            fn eq(&self, other: &&str) -> bool { self.$field == *other }
        }
    )*};
}

/// An enum whose variants each stand for one string, with a `from_table`
/// lookup from the string to the variant, written as `"S" => Subject`.
macro_rules! relations {
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            $($(#[$variant_attr:meta])* $code:literal => $variant:ident,)*
        }
    ) => {
        $(#[$attr])*
        pub enum $name {
            $($(#[$variant_attr])* $variant,)*
        }

        impl $name {
            /// The variant that stands for `code`, if one does.
            fn from_table(code: &str) -> Option<Self> {
                match code {
                    $($code => Some(Self::$variant),)*
                    _ => None,
                }
            }
        }
    };
}
