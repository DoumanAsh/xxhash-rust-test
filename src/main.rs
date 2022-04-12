#[macro_use]
extern crate quickcheck_macros;

mod quickcheck;

use criterion::Criterion;

const DATA: [&str; 34] = [
    "waifulandshigtgsqwetyuop[]asbnm,./",
    "waifulandshigtgsqwetyuop[]asbnm,.",
    "waifulandshigtgsqwetyuop[]asbnm,",
    "waifulandshigtgsqwetyuop[]asbnm",
    "waifulandshigtgsqwetyuop[]asbn",
    "waifulandshigtgsqwetyuop[]asb",
    "waifulandshigtgsqwetyuop[]as",
    "waifulandshigtgsqwetyuop[]a",
    "waifulandshigtgsqwetyuop[]",
    "waifulandshigtgsqwetyuop[",
    "waifulandshigtgsqwetyuop",
    "waifulandshigtgsqwetyuo",
    "waifulandshigtgsqwetyu",
    "waifulandshigtgsqwety",
    "waifulandshigtgsqwet",
    "waifulandshigtgsqwe",
    "waifulandshigtgsqw",
    "waifulandshigtgsq",
    "waifulandshigtgs",
    "waifulandshigtg",
    "waifulandshigt",
    "waifulandshig",
    "waifulandshi",
    "waifulandsh",
    "waifulands",
    "waifuland",
    "waifulan",
    "waifula",
    "waiful",
    "lolka",
    "lolk",
    "lol",
    "lo",
    "l",
];

fn against_c(c: &mut Criterion) {
    c.bench_function("xxh32 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        xxhash_rust::xxh32::xxh32(input.as_bytes(), 0);
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh32 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        xxhash_rust::const_xxh32::xxh32(input.as_bytes(), 0);
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh32 Rust Stateful", |b| b.iter_batched(|| &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh32::Xxh32::new(0);
        hasher.update(input.as_bytes());
        hasher.digest();
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        xxhash_rust::xxh64::xxh64(input.as_bytes(), 0);
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh64 Rust Stateful", |b| b.iter_batched(|| &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh64::Xxh64::new(0);
        hasher.update(input.as_bytes());
        hasher.digest();
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        xxhash_rust::const_xxh64::xxh64(input.as_bytes(), 0);
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
        xxhash_rust::xxh3::xxh3_64(input.as_bytes());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 Rust Stateful", move |b| b.iter_batched(move || &DATA, |data| for input in data {
        let mut hasher = xxhash_rust::xxh3::Xxh3::new();
        hasher.update(input.as_bytes());
        hasher.digest();
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        xxhash_rust::xxh3::xxh3_128(input.as_bytes());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh3 64 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        xxhash_rust::const_xxh3::xxh3_64(input.as_bytes());
    }, criterion::BatchSize::SmallInput));

    c.bench_function("const_xxh3 128 Rust", |b| b.iter_batched(|| &DATA, |data| for input in data {
        xxhash_rust::const_xxh3::xxh3_128(input.as_bytes());
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

    let mut rand_230_bytes = [0u8; 260];
    let _ = getrandom::getrandom(&mut rand_230_bytes);

    c.bench_function("xxh3_64 C 230b", |b| b.iter_batched(|| rand_230_bytes, |input| {
        unsafe {
            xxhash_c_sys::XXH3_64bits(input.as_ptr() as _, input.len());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 Rust 230b", |b| b.iter_batched(|| rand_230_bytes, |input| {
        xxhash_rust::xxh3::xxh3_64(&input);
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 C 230b", |b| b.iter_batched(|| rand_230_bytes, |input| {
        unsafe {
            xxhash_c_sys::XXH3_128bits(input.as_ptr() as _, input.len());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 Rust 230b", |b| b.iter_batched(|| rand_230_bytes, |input| {
        xxhash_rust::xxh3::xxh3_128(&input);
    }, criterion::BatchSize::SmallInput));

    let mut rand_260_bytes = [0u8; 260];
    let _ = getrandom::getrandom(&mut rand_260_bytes);

    c.bench_function("xxh3_64 C 260b", |b| b.iter_batched(|| rand_260_bytes, |input| {
        unsafe {
            xxhash_c_sys::XXH3_64bits(input.as_ptr() as _, input.len());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 C Stateful 260b", |b| b.iter_batched(|| rand_260_bytes, |input| {
        use xxhash_c_sys as sys;

        let mut state = core::mem::MaybeUninit::<sys::XXH3_state_t>::uninit();

        unsafe {
            sys::XXH3_64bits_reset(state.as_mut_ptr());
            sys::XXH3_64bits_update(state.as_mut_ptr(), input.as_ptr() as _, input.len());
            sys::XXH3_64bits_digest(state.as_mut_ptr());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 C 260b", |b| b.iter_batched(|| rand_260_bytes, |input| {
        unsafe {
            xxhash_c_sys::XXH3_128bits(input.as_ptr() as _, input.len());
        }
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 Rust 260b", |b| b.iter_batched(|| rand_260_bytes, |input| {
        xxhash_rust::xxh3::xxh3_64(&input);
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_64 Rust Stateful 260b", |b| b.iter_batched(|| rand_260_bytes, |input| {
        let mut hasher = xxhash_rust::xxh3::Xxh3::new();
        hasher.update(&input);
        hasher.digest();
    }, criterion::BatchSize::SmallInput));

    c.bench_function("xxh3_128 Rust 260b", |b| b.iter_batched(|| rand_260_bytes, |input| {
        xxhash_rust::xxh3::xxh3_128(&input);
    }, criterion::BatchSize::SmallInput));
}

fn main() {
  let mut criterion = Criterion::default().configure_from_args();
  against_c(&mut criterion);
  Criterion::default().configure_from_args().final_summary();
}
