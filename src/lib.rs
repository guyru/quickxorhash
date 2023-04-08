//! [QuickXorHash] implementation in Rust.
//!
//! A quick, simple non-cryptographic hash algorithm that works by XORing the bytes in a
//! # QuickXorHash
//! circular-shifting fashion.
//!
//! [QuickXorHash]: https://learn.microsoft.com/en-us/onedrive/developer/code-snippets/quickxorhash
//!
//! ## Examples
//! ```
//! use quickxorhash::QuickXorHash;
//!
//! let mut qx = QuickXorHash::new();
//! qx.update(b"hello world");
//! assert_eq!(qx.finalize(), [104, 40, 3, 27, 216, 240, 6, 16, 220, 225, 13, 114, 107, 3, 25, 0, 0, 0, 0, 0]);
//! ```

const WIDTH_IN_BYTES: usize = 160 / 8;
const SHIFT: usize = 11;

pub struct QuickXorHash {
    data: [u8; WIDTH_IN_BYTES],
    length: usize,
    shift: usize,
}

impl Default for QuickXorHash {
    fn default() -> Self {
        QuickXorHash::new()
    }
}

impl QuickXorHash {
    pub fn new() -> Self {
        QuickXorHash {
            data: [0; WIDTH_IN_BYTES],
            length: 0,
            shift: 0,
        }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        for b in bytes {
            let shift_bytes = self.shift / 8;
            let shift_bits = self.shift % 8;
            let byte_h = match shift_bits {
                0 => 0,
                _ => b >> (8 - shift_bits),
            };
            let byte_l = b << shift_bits;
            self.data[WIDTH_IN_BYTES - 1 - (shift_bytes % WIDTH_IN_BYTES)] ^= byte_l;
            self.data[WIDTH_IN_BYTES - 1 - ((shift_bytes + 1) % WIDTH_IN_BYTES)] ^= byte_h;
            self.shift += SHIFT;
        }
        self.length += bytes.len();
    }

    pub fn finalize(&mut self) -> [u8; WIDTH_IN_BYTES] {
        let mut out = self.data;
        out.reverse();

        // xor the length
        let length_bytes = self.length.to_le_bytes();
        let len = length_bytes.len();
        for i in 0..len {
            out[WIDTH_IN_BYTES - len + i] ^= length_bytes[i];
        }

        out
    }
}
