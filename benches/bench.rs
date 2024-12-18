use criterion::{Criterion, black_box};

use get_random_const::random;

const DATA: [&[u8]; 12] = [
    &random!([u8; 1]),
    &random!([u8; 3]),
    &random!([u8; 7]),
    &random!([u8; 15]),
    &random!([u8; 31]),
    &random!([u8; 64]),
    &random!([u8; 127]),
    &random!([u8; 255]),
    &random!([u8; 511]),
    &random!([u8; 1023]),
    &random!([u8; 2047]),
    &random!([u8; 4097]),
];

fn against_c(c: &mut Criterion) {
    c.bench_function("xxh32 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::xxh32::xxh32(input, 0));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh32 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::const_xxh32::xxh32(input, 0));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh32 Rust Stateful", |b| b.iter_batched(|| &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh32::Xxh32::new(0);
        hasher.update(black_box(input));
        black_box(hasher.digest());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::xxh64::xxh64(input, 0));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh64 Rust Stateful", |b| b.iter_batched(|| &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh64::Xxh64::new(0);
        hasher.update(black_box(input));
        black_box(hasher.digest());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::const_xxh64::xxh64(input, 0));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh32 C", |b| b.iter_batched(|| &DATA, |data| for input in data {
        unsafe {
            xxhash_c_sys::XXH32(input.as_ptr() as _, input.len(), 0);
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh32 C Stateful", |b| b.iter_batched(|| &DATA, |data| for input in data {
        use xxhash_c_sys as sys;

        let mut state = core::mem::MaybeUninit::<sys::XXH32_state_t>::uninit();

        unsafe {
            sys::XXH32_reset(state.as_mut_ptr(), 0);
            sys::XXH32_update(state.as_mut_ptr(), input.as_ptr() as _, input.len());
            sys::XXH32_digest(state.as_mut_ptr());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh64 C", |b| b.iter_batched(|| &DATA, |data| for input in data {
        unsafe {
            xxhash_c_sys::XXH64(input.as_ptr() as _, input.len(), 0);
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh64 C Stateful", |b| b.iter_batched(|| &DATA, |data| for input in data {
        use xxhash_c_sys as sys;

        let mut state = core::mem::MaybeUninit::<sys::XXH64_state_t>::uninit();

        unsafe {
            sys::XXH64_reset(state.as_mut_ptr(), 0);
            sys::XXH64_update(state.as_mut_ptr(), input.as_ptr() as _, input.len());
            sys::XXH64_digest(state.as_mut_ptr());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::xxh3::xxh3_64(input));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 Rust Stateful", move |b| b.iter_batched(move || &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh3::Xxh3::new();
        hasher.update(black_box(input));
        black_box(hasher.digest());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("twox-hash xxh3_64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(twox_hash::XxHash3_64::oneshot(input));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("twox-hash xxh3_64 Rust Stateful", move |b| b.iter_batched(move || &DATA, |data| for input in data {
        use core::hash::Hasher;

        let mut hasher = twox_hash::XxHash3_64::new();
        hasher.write(black_box(input));
        black_box(hasher.finish());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 Rust Default Stateful", move |b| b.iter_batched(move || &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh3::Xxh3Default::new();
        hasher.update(black_box(input));
        black_box(hasher.digest());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::xxh3::xxh3_128(input));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 Rust Stateful", move |b| b.iter_batched(move || &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh3::Xxh3::new();
        hasher.update(black_box(input));
        black_box(hasher.digest128());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("twox-hash xxh3_128 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(twox_hash::XxHash3_128::oneshot(input));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("twox-hash xxh3_128 Rust Stateful", move |b| b.iter_batched(move || &DATA, |data| for input in data {
        let mut hasher = twox_hash::XxHash3_128::new();
        hasher.write(black_box(input));
        black_box(hasher.finish_128());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 Rust Default Stateful", move |b| b.iter_batched(move || &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh3::Xxh3Default::new();
        hasher.update(black_box(input));
        black_box(hasher.digest128());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 C Stateful", |b| b.iter_batched(|| &DATA, |data| for input in data {
        use xxhash_c_sys as sys;

        let mut state = core::mem::MaybeUninit::<sys::XXH3_state_t>::uninit();

        unsafe {
            sys::XXH3_64bits_reset(state.as_mut_ptr());
            sys::XXH3_64bits_update(state.as_mut_ptr(), input.as_ptr() as _, input.len());
            sys::XXH3_64bits_digest(state.as_mut_ptr());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 C", |b| b.iter_batched(|| &DATA, |data| for input in data {
        unsafe {
            xxhash_c_sys::XXH3_64bits(input.as_ptr() as _, input.len());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 C", |b| b.iter_batched(|| &DATA, |data| for input in data {
        unsafe {
            xxhash_c_sys::XXH3_128bits(input.as_ptr() as _, input.len());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh3 64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::const_xxh3::xxh3_64(input));
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh3 128 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        black_box(xxhash_rust::const_xxh3::xxh3_128(input));
    }, criterion::BatchSize::SmallInput));
}

fn main() {
  let mut criterion = Criterion::default().configure_from_args();
  against_c(&mut criterion);
  Criterion::default().configure_from_args().final_summary();
}
