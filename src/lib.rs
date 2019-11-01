//! Belene provides little endian and big endian versions of the basic integral
//! types.
//!
//! These types are aligned like their native endian versions. You might want to
//! use #[repr(packed)] for your type definitions because of it.

use std::fmt;

macro_rules! impl_types {
    ($(
        $be: ident, $le: ident, $ne: ident,
    )*) => {
        $(
            #[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            pub struct $be($ne);

            impl $be {
                #[inline]
                pub const fn from_ne(val: $ne) -> Self {
                    Self(val.to_be())
                }

                #[inline]
                pub const fn to_ne(self) -> $ne {
                    $ne::from_be(self.0)
                }
            }

            impl fmt::Debug for $be {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.to_ne().fmt(f)
                }
            }

            #[derive(Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            pub struct $le($ne);

            impl $le {
                #[inline]
                pub const fn from_ne(val: $ne) -> Self {
                    Self(val.to_le())
                }

                #[inline]
                pub const fn to_ne(self) -> $ne {
                    $ne::from_le(self.0)
                }
            }

            impl fmt::Debug for $le {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.to_ne().fmt(f)
                }
            }
        )*
    }
}

impl_types!(
    u16be, u16le, u16,
    u32be, u32le, u32,
    u64be, u64le, u64,
    u128be, u128le, u128,
    i16be, i16le, i16,
    i32be, i32le, i32,
    i64be, i64le, i64,
    i128be, i128le, i128,
);
