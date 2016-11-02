#![feature(test)]

extern crate test;
extern crate base58;
extern crate rbase58;
extern crate rust_base58;

macro_rules! bench_from {
    ($krate:ident, $value:expr) => {
        #[bench]
        fn $krate(b: &mut ::test::Bencher) {
            use $krate::FromBase58;
            let temp = $value;
            if let Err(_) = temp.from_base58() {
                b.iter(|| ());
            } else {
                b.iter(|| temp.from_base58().unwrap());
            }
        }
    };
}

macro_rules! bench_to {
    ($krate:ident, $value:expr) => {
        #[bench]
        fn $krate(b: &mut ::test::Bencher) {
            use $krate::ToBase58;
            let temp = $value;
            b.iter(|| temp.to_base58());
        }
    };
}

macro_rules! case {
    ($n:ident $v:expr => $s:expr) => {
        mod $n {
            mod decode {
                bench_from!(base58, $s);
                bench_from!(rbase58, $s);
                bench_from!(rust_base58, $s);
            }
            mod encode {
                bench_to!(base58, $v);
                bench_to!(rbase58, $v);
                bench_to!(rust_base58, $v);
            }
        }
    }
}

case! { a_empty vec![] => "" }
case! { b_1_byte vec![0x61] => "2g" }
case! { c_5_bytes vec![0x51, 0x6b, 0x6f, 0xcd, 0x0f] => "ABnLTmg" }
case! { d_10_bytes
    vec![0xec, 0xac, 0x89, 0xca, 0xd9, 0x39, 0x23, 0xc0, 0x23, 0x21]
        => "EJDM8drfXA6uyA"
}
case! { e_10_bytes_zero
    vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        => "1111111111"
}
case! { f_10_bytes_max
    vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
        => "FPBt6CHo3fovdL"
}
case! { g_37_bytes
    vec![
        0x80, 0x11, 0x84, 0xcd, 0x2c, 0xdd, 0x64, 0x0c, 0xa4, 0x2c, 0xfc, 0x3a,
        0x09, 0x1c, 0x51, 0xd5, 0x49, 0xb2, 0xf0, 0x16, 0xd4, 0x54, 0xb2, 0x77,
        0x40, 0x19, 0xc2, 0xb2, 0xd2, 0xe0, 0x85, 0x29, 0xfd, 0x20, 0x6e, 0xc9,
        0x7e
    ] => "5Hx15HFGyep2CfPxsJKe2fXJsCVn5DEiyoeGGF6JZjGbTRnqfiD"
}
case! { h_256_bytes
    vec![
        0x65, 0x5f, 0x65, 0x20, 0xc4, 0xd8, 0xa5, 0x86, 0xce, 0x80, 0x1a, 0x4e,
        0x60, 0x73, 0x91, 0x40, 0x10, 0x8f, 0xd5, 0xdc, 0x5b, 0x3e, 0x8e, 0x08,
        0x47, 0x98, 0x82, 0xc6, 0x29, 0xee, 0x49, 0x8d, 0xb6, 0x41, 0xa1, 0xc6,
        0xa9, 0xd3, 0x63, 0xcb, 0xe2, 0x4e, 0x3f, 0x90, 0x78, 0x04, 0xf4, 0x49,
        0x5c, 0x4b, 0x39, 0x73, 0x9b, 0x5c, 0x4b, 0x9f, 0x23, 0xde, 0xc4, 0x8a,
        0x3d, 0xb8, 0x1a, 0x6c, 0xfd, 0x5a, 0xc1, 0xe3, 0x28, 0x9a, 0xf6, 0x72,
        0xfb, 0x2d, 0x33, 0x9d, 0xb6, 0xc4, 0x38, 0xfa, 0x8d, 0x16, 0xc9, 0x0d,
        0x00, 0xab, 0xc7, 0x9a, 0x27, 0xd2, 0x8e, 0x45, 0xdc, 0x49, 0x8d, 0xf9,
        0x80, 0x86, 0x11, 0x91, 0x86, 0x98, 0xcc, 0xc2, 0x6e, 0x85, 0xd2, 0x38,
        0xfc, 0xff, 0x66, 0xf0, 0x9d, 0x7d, 0xa5, 0x4c, 0x6f, 0x0d, 0xe5, 0xd0,
        0x60, 0x6c, 0xe7, 0x31, 0x38, 0xa0, 0x86, 0xde, 0x24, 0x28, 0x05, 0x6c,
        0x03, 0xb6, 0x21, 0xde, 0xaa, 0x8b, 0x81, 0xcc, 0xb6, 0x0e, 0x19, 0xdc,
        0xe5, 0x50, 0xb5, 0xb7, 0x6e, 0x8f, 0x22, 0xa7, 0x6f, 0x86, 0x75, 0x06,
        0xb8, 0xca, 0xa0, 0xc6, 0x29, 0x8f, 0xf6, 0xc4, 0x8b, 0x22, 0x24, 0xc0,
        0xf7, 0x09, 0x10, 0x6f, 0x10, 0x8a, 0xc2, 0x57, 0x90, 0x50, 0x62, 0x9e,
        0x95, 0x4c, 0x47, 0x79, 0xdb, 0xc9, 0x82, 0x9f, 0x45, 0xac, 0x8b, 0x31,
        0xa4, 0xfb, 0x6b, 0xdd, 0x86, 0x7f, 0x9b, 0x6f, 0x48, 0xe4, 0x34, 0x84,
        0x0c, 0x45, 0x6c, 0xfa, 0xa3, 0x14, 0x52, 0x22, 0x46, 0xf9, 0x20, 0x5f,
        0x6a, 0xb4, 0x25, 0x09, 0xb1, 0xae, 0x04, 0x3f, 0x27, 0xa0, 0xda, 0xb6,
        0x91, 0x45, 0x09, 0x37, 0xf1, 0x17, 0x2d, 0xb8, 0xa8, 0xaa, 0x5a, 0x61,
        0xf1, 0xbe, 0x08, 0x40, 0x47, 0xa8, 0x16, 0xf9, 0xb0, 0x0f, 0x6d, 0x34,
        0x62, 0x29, 0x2b, 0xb3
    ] => "\
        5gkXES6JSFLhJ3pkwQsV3MT3TBjsW5vQnAW8CwPLS1oDsJgjq8dchz994yCJHD1C16k3Pk\
        Gp8o61dMfXy1vVwXcD147ix2BXD87xcXGnzB4mxaUEvgqDonZz8xQE9XL44XvLQshJw7kp\
        54MkSPbVkxvzKdxiYHkgAjLfmx5wdyDNjPu2DUYmxRrTtjDw5QVMaqAp3fLrQ6GnXuhZmB\
        Jdj8rTprjADLM5tox6tHgyj2bm37ECxKevEapzy4nDGmZrzMubp9s58TsV1wk3LUQsRF49\
        L9NzDatxVUetHTjQennpEHEuMTU9D8GM6De44J7Sk5fnJGh614ZtmrYyFcCE3X5mdTwaxA\
    "
}
