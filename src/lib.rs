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
//!
//! ## Performance
//! The best Performance is achieved when updating large chunks which are multiples of BLOCK_SIZE
//! (160).

const WIDTH_IN_BYTES: usize = 160 / 8;
const SHIFT: usize = 11;
pub const BLOCK_SIZE: usize = 160;

pub struct QuickXorHash {
    data: [u8; BLOCK_SIZE],
    length: usize,
    index: usize,
}

impl Default for QuickXorHash {
    fn default() -> Self {
        QuickXorHash::new()
    }
}

impl QuickXorHash {
    pub fn new() -> Self {
        QuickXorHash {
            data: [0; BLOCK_SIZE],
            length: 0,
            index: 0,
        }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        let prefix = std::cmp::min((BLOCK_SIZE - self.index) % BLOCK_SIZE, bytes.len());

        self.data[self.index..]
            .iter_mut()
            .zip(bytes[..prefix].iter())
            .for_each(|(d, b)| *d ^= *b);
        self.index = (self.index + prefix) % BLOCK_SIZE;

        if bytes.len() > prefix {
            debug_assert!(self.index == 0);

            // The following loop should be heavily optimized by the compiler
            let chunks = bytes[prefix..].chunks_exact(BLOCK_SIZE);
            let tail = chunks.remainder();
            for block in chunks {
                self.data
                    .iter_mut()
                    .zip(block.iter())
                    .for_each(|(d, b)| *d ^= *b);
            }

            self.data.iter_mut().zip(tail).for_each(|(d, b)| *d ^= *b);
            self.index += tail.len();
        }
        self.length += bytes.len();
    }

    pub fn finalize(&mut self) -> [u8; WIDTH_IN_BYTES] {
        let mut out = [0_u8; WIDTH_IN_BYTES];
        let mut shift = 0;
        for b in self.data {
            let shift_bytes = shift / 8;
            let shift_bits = shift % 8;
            let byte_h = match shift_bits {
                0 => 0,
                _ => b >> (8 - shift_bits),
            };
            let byte_l = b << shift_bits;
            out[(shift_bytes % WIDTH_IN_BYTES)] ^= byte_l;
            out[(shift_bytes + 1) % WIDTH_IN_BYTES] ^= byte_h;
            shift += SHIFT;
        }

        // xor the length
        let length_bytes = self.length.to_le_bytes();
        let len = length_bytes.len();
        for i in 0..len {
            out[WIDTH_IN_BYTES - len + i] ^= length_bytes[i];
        }

        out
    }
}
