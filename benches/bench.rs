#[macro_use]
extern crate bencher;
use quickxorhash::*;

use bencher::Bencher;

fn bench_small(bench: &mut Bencher) {
    const N: usize = 1024;
    let bytes: [u8; N] = core::array::from_fn(|i| (i * 7) as u8);
    bench.iter(|| {
        let mut qx = QuickXorHash::new();
        qx.update(&bytes);
        qx.finalize();
    });
    bench.bytes = N as u64;
}

fn bench_large(bench: &mut Bencher) {
    const N: usize = 1024 * 1024;
    let bytes: [u8; N] = core::array::from_fn(|i| (i * 7) as u8);
    bench.iter(|| {
        let mut qx = QuickXorHash::new();
        qx.update(&bytes);
        qx.finalize();
    });
    bench.bytes = N as u64;
}

/// Test the worst case scenario in terms of optimization.
fn bench_unaligned(bench: &mut Bencher) {
    const N: usize = 1024 * 1024;
    let bytes: [u8; N] = core::array::from_fn(|i| (i * 7) as u8);
    bench.iter(|| {
        let mut qx = QuickXorHash::new();
        for block in bytes[..].chunks(BLOCK_SIZE-1) {
            qx.update(block);
        }
        qx.finalize();
    });
    bench.bytes = N as u64;
}

benchmark_group!(benches, bench_small, bench_large, bench_unaligned);
benchmark_main!(benches);
