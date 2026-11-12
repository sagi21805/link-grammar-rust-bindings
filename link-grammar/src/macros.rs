/// `as_str`, `Display` and comparisons with strings for types that wrap a
/// string borrowed from a linkage, as `Type<'l>(&'l str)`.
macro_rules! borrowed_str {
    ($($ty:ident),*) => {$(
        impl<'l> $ty<'l> {
            #[doc = concat!("The ", stringify!($ty), " as link-grammar prints it.")]
            pub fn as_str(self) -> &'l str { self.0 }
        }

        impl ::std::fmt::Display for $ty<'_> {
            fn fmt(
                &self,
                f: &mut ::std::fmt::Formatter<'_>,
            ) -> ::std::fmt::Result {
                f.write_str(self.0)
            }
        }

        impl PartialEq<str> for $ty<'_> {
            fn eq(&self, other: &str) -> bool { self.0 == other }
        }

        impl PartialEq<&str> for $ty<'_> {
            fn eq(&self, other: &&str) -> bool { self.0 == *other }
        }

        const _: () = assert!(
            ::std::mem::size_of::<$ty<'_>>() == ::std::mem::size_of::<&str>(),
            concat!(stringify!($ty), " is a `&str` and nothing more"),
        );
    )*};
}
