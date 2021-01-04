use std::io;
use std::vec::Vec;
use std::num::Wrapping;
use byteorder::{ByteOrder, LittleEndian};
use std::error::Error;
use crate::config::MD5_SALT;

struct MD5State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

struct MD5Util {}

impl MD5Util {
    fn get_round_shift() -> [u8; 64] {
        [7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
            5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
            4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21]
    }

    fn get_padding() -> [u8; 64] {
        [
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ]
    }

    fn get_constants() -> [u32; 64] {
        [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
        ]
    }
}

impl MD5State {
    fn get_state() -> MD5State {
        MD5State {
            a: 0x67452301,
            b: 0xefcdab89,
            c: 0x98badcfe,
            d: 0x10325476,
        }
    }
}

pub struct MD5 {}

impl MD5 {
    fn padding(input: &str) -> Vec<u8> {
        let mut msg = input.as_bytes().to_vec();

        let msg_length: u64 = input.as_bytes().len() as u64 * 8;
        let size = input.as_bytes().len() % 64;

        let range = if size >= 56 {
            120 - size
        } else {
            56 - size
        };

        msg.extend_from_slice(&MD5Util::get_padding()[0..range]);

        let mut length: [u8; 8] = [0; 8];
        LittleEndian::write_u64(&mut length, msg_length);

        msg.extend_from_slice(&length);

        msg
    }

    fn round_f(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>, index: i32) -> (Wrapping<u32>, i32) {
        macro_rules! F {
            ($x:expr, $y:expr, $z:expr) => (($x & $y) | (!$x & $z));
        }

        (F!(x, y, z), index)
    }

    fn round_g(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>, index: i32) -> (Wrapping<u32>, i32) {
        macro_rules! G {
            ($x:expr, $y:expr, $z:expr) => (($x & $z) | ($y & !$z));
        }

        (G!(x, y, z), (5 * index + 1) % 16)
    }

    fn round_h(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>, index: i32) -> (Wrapping<u32>, i32) {
        macro_rules! H {
            ($x:expr, $y:expr, $z:expr) => ($x ^ $y ^ $z);
        }

        (H!(x, y, z), (3 * index + 5) % 16)
    }

    fn round_i(x: Wrapping<u32>, y: Wrapping<u32>, z: Wrapping<u32>, index: i32) -> (Wrapping<u32>, i32) {
        macro_rules! I {
            ($x:expr, $y:expr, $z:expr) => ($y ^ ($x | !$z));
        }

        (I!(x, y, z), (7 * index) % 16)
    }

    pub fn encode(input: &str) -> String {
        let state = MD5State::get_state();
        let constants = MD5Util::get_constants();
        let round_shift = MD5Util::get_round_shift();

        let mut a0 = Wrapping(state.a);
        let mut b0 = Wrapping(state.b);
        let mut c0 = Wrapping(state.c);
        let mut d0 = Wrapping(state.d);

        let msg = MD5::padding(input);
        let msg_range = msg.len() / 64;

        for i in 0..msg_range {
            let mut A = Wrapping(a0.0);
            let mut B = Wrapping(b0.0);
            let mut C = Wrapping(c0.0);
            let mut D = Wrapping(d0.0);

            for k in 0..64 {
                let mut f = Wrapping(0u32);
                let mut g = 0;

                if 0 <= k && k < 16 {
                    let (p, q) = MD5::round_f(B, C, D, k);
                    f = p;
                    g = q;
                } else if 16 <= k && k < 32 {
                    let (p, q) = MD5::round_g(B, C, D, k);
                    f = p;
                    g = q;
                } else if 32 <= k && k < 48 {
                    let (p, q) = MD5::round_h(B, C, D, k);
                    f = p;
                    g = q;
                } else if 48 <= k && k < 64 {
                    let (p, q) = MD5::round_i(B, C, D, k);
                    f = p;
                    g = q;
                }

                f += A + Wrapping(constants[k as usize]);
                f += Wrapping(LittleEndian::read_u32(&msg[(g * 4) as usize..(g * 4 + 4) as usize]));
                A = D;
                D = C;
                C = B;
                B += Wrapping(f.0.rotate_left(round_shift[k as usize] as u32));
            }
            a0 += A;
            b0 += B;
            c0 += C;
            d0 += D;
        }

        let digest = [a0.0, b0.0, c0.0, d0.0];
        digest.iter().map(|d| d.to_ne_bytes().iter().map(|b| format!("{:02x}", b)).collect::<Vec<String>>().join("")).collect::<Vec<String>>().join("")
    }
}

//密码验证
pub fn verify(pass: &str, newPass: &str) -> bool {
    let mut ss=newPass.to_owned();
    ss.push_str(MD5_SALT);
    return if pass.eq(&MD5::encode(&ss)) {
        true
    } else {
        false
    };
}