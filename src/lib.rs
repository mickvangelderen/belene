//! Belene provides little endian and big endian versions of the basic integral
//! types.
//!
//! The types have byte alignment so they can be used in types that are
//! transmuted or cast from bytes.

use std::fmt;

macro_rules! impl_types {
    ($(
        $be: ident, $le: ident, $ne: ident, [u8; $n: expr],
    )*) => {
        $(
            #[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            pub struct $be(pub [u8; $n]);

            impl $be {
                #[inline]
                pub const fn to_bytes(&self) -> [u8; $n] {
                    self.0
                }

                #[inline]
                pub fn from_ne(val: $ne) -> Self {
                    Self(val.to_be_bytes())
                }

                #[inline]
                pub fn to_ne(self) -> $ne {
                    $ne::from_be_bytes(self.0)
                }
            }

            impl fmt::Debug for $be {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    self.to_ne().fmt(f)
                }
            }

            #[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
            #[allow(non_camel_case_types)]
            #[repr(transparent)]
            pub struct $le(pub [u8; $n]);

            impl $le {
                #[inline]
                pub const fn to_bytes(&self) -> [u8; $n] {
                    self.0
                }

                #[inline]
                pub fn from_ne(val: $ne) -> Self {
                    Self(val.to_le_bytes())
                }

                #[inline]
                pub fn to_ne(self) -> $ne {
                    $ne::from_le_bytes(self.0)
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
    u16be, u16le, u16, [u8; 2],
    u32be, u32le, u32, [u8; 4],
    u64be, u64le, u64, [u8; 8],
    u128be, u128le, u128, [u8; 16],
    i16be, i16le, i16, [u8; 2],
    i32be, i32le, i32, [u8; 4],
    i64be, i64le, i64, [u8; 8],
    i128be, i128le, i128, [u8; 16],
);
