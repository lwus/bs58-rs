#![warn(fat_ptr_transmutes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_results)]
#![warn(variant_size_differences)]

#![allow(unknown_lints)] // For clippy

//! Another [Base58][] codec implementation.
//!
//! Compared to [`base58`][] this is significantly faster at decoding (about
//! 2.4x as fast when decoding 32 bytes), almost the same speed for encoding
//! (about 3% slower when encoding 32 bytes), doesn't have the 128 byte
//! limitation and supports a configurable alphabet.
//!
//! Compared to [`rust-base58`][] this is massively faster (over ten times as
//! fast when decoding 32 bytes, almost 40 times as fast when encoding 32
//! bytes), has no external dependencies and supports a configurable alphabet.
//!
//! [Base58]: https://en.wikipedia.org/wiki/Base58
//! [`base58`]: https://github.com/debris/base58
//! [`rust-base58`]: https://github.com/nham/rust-base58

mod decode;
mod encode;
pub mod alphabet;

#[cfg(test)]
const TEST_CASES: &'static [(&'static [u8], &'static str)] = &[
    (&[], ""),
    (&[0x61], "2g"),
    (&[0x62, 0x62, 0x62], "a3gV"),
    (&[0x63, 0x63, 0x63], "aPEr"),
    (&[0x57, 0x2e, 0x47, 0x94], "3EFU7m"),
    (&[0x10, 0xc8, 0x51, 0x1e], "Rt5zm"),
    (&[0x51, 0x6b, 0x6f, 0xcd, 0x0f], "ABnLTmg"),
    (&[0xbf, 0x4f, 0x89, 0x00, 0x1e, 0x67, 0x02, 0x74, 0xdd], "3SEo3LWLoPntC"),
    (&[0xec, 0xac, 0x89, 0xca, 0xd9, 0x39, 0x23, 0xc0, 0x23, 0x21], "EJDM8drfXA6uyA"),
    (&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], "1111111111"),
    (&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "FPBt6CHo3fovdL"),
    (&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "NKioeUVktgzXLJ1B3t"),
    (&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], "YcVfxkQb6JRzqk5kF2tNLv"),
    (&[0x73, 0x69, 0x6d, 0x70, 0x6c, 0x79, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67], "2cFupjhnEsSn59qHXstmK2ffpLv2"),
    (&[0x00, 0xeb, 0x15, 0x23, 0x1d, 0xfc, 0xeb, 0x60, 0x92, 0x58, 0x86, 0xb6, 0x7d, 0x06, 0x52, 0x99, 0x92, 0x59, 0x15, 0xae, 0xb1, 0x72, 0xc0, 0x66, 0x47], "1NS17iag9jJgTHD1VXjvLCEnZuQ3rJDE9L"),
    (&[0x00, 0x3c, 0x17, 0x6e, 0x65, 0x9b, 0xea, 0x0f, 0x29, 0xa3, 0xe9, 0xbf, 0x78, 0x80, 0xc1, 0x12, 0xb1, 0xb3, 0x1b, 0x4d, 0xc8, 0x26, 0x26, 0x81, 0x87], "16UjcYNBG9GTK4uq2f7yYEbuifqCzoLMGS"),
    (&[0x80, 0x11, 0x84, 0xcd, 0x2c, 0xdd, 0x64, 0x0c, 0xa4, 0x2c, 0xfc, 0x3a, 0x09, 0x1c, 0x51, 0xd5, 0x49, 0xb2, 0xf0, 0x16, 0xd4, 0x54, 0xb2, 0x77, 0x40, 0x19, 0xc2, 0xb2, 0xd2, 0xe0, 0x85, 0x29, 0xfd, 0x20, 0x6e, 0xc9, 0x7e], "5Hx15HFGyep2CfPxsJKe2fXJsCVn5DEiyoeGGF6JZjGbTRnqfiD"),
];

pub use decode::{ FromBase58, FromBase58Error };
pub use encode::ToBase58;
