/// A link-grammar language's relations, one variant per link type in
/// number order, plus `Idiom` for every `_I` type.
macro_rules! link_relations {
    (
        $(#[$attr:meta])*
        pub enum $name:ident {
            $($code:literal => $variant:ident,)*
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)*
            /// `_I`, the words of an idiom. link-grammar generates one
            /// link type per idiom, such as `_IBBN`.
            Idiom,
        }

        impl $name {
            /// Every variant but `Idiom`, in number order.
            const ALL: &[Self] = &[$(Self::$variant,)*];
            /// The code of each variant, `Idiom`'s last.
            const CODES: &[&str] = &[$($code,)* "_I"];
        }

        impl $crate::LinkRelation for $name {
            fn from_number(number: u32) -> Self {
                // Idiom types are numbered after the rest.
                Self::ALL
                    .get(number as usize)
                    .copied()
                    .unwrap_or(Self::Idiom)
            }

            fn code(self) -> &'static str { Self::CODES[self as usize] }
        }
    };
}
