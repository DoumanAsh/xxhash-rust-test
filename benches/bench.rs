use criterion::Criterion;

use get_random_const::random;

const DATA_3: &[[u8; 3]] = &[
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
    random!([u8; 3]),
];

const DATA_6: &[[u8; 6]] = &[
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
    random!([u8; 6]),
];

const DATA_9: &[[u8; 9]] = &[
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
    random!([u8; 9]),
];

const DATA_16: &[[u8; 16]] = &[
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
    random!([u8; 16]),
];

const DATA_17: &[[u8; 17]] = &[
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
    random!([u8; 17]),
];

const DATA_64: &[[u8; 64]] = &[
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
    random!([u8; 64]),
];

const DATA_128: &[[u8; 128]] = &[
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
    random!([u8; 128]),
];

const DATA_149: &[[u8; 149]] = &[
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
    random!([u8; 149]),
];

const DATA_240: &[[u8; 240]] = &[
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
    random!([u8; 240]),
];

const DATA_2046: &[[u8; 2046]] = &[
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
    random!([u8; 2046]),
];

mod runners {
    use criterion::{BenchmarkGroup, Throughput};
    use criterion::measurement::Measurement;
    use std::hint::black_box;

    pub fn xxh3_seedless_oneshot64<const SIZE: usize>(group: &mut BenchmarkGroup<'_, impl Measurement>, data: &[[u8; SIZE]]) {
        group.throughput(Throughput::Bytes(SIZE as _));
        group.bench_function(&format!("Rust/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            black_box(xxhash_rust::xxh3::xxh3_64(input.as_slice()));
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("Twox/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            black_box(twox_hash::XxHash3_64::oneshot(input.as_slice()));
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("C/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            unsafe {
                xxhash_c_sys::XXH3_64bits(input.as_ptr() as _, input.len());
            }
        }, criterion::BatchSize::SmallInput));
    }

    pub fn xxh3_seedless_oneshot128<const SIZE: usize>(group: &mut BenchmarkGroup<'_, impl Measurement>, data: &[[u8; SIZE]]) {
        group.throughput(Throughput::Bytes(SIZE as _));
        group.throughput(Throughput::Bytes(SIZE as _));
        group.bench_function(&format!("Rust/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            black_box(xxhash_rust::xxh3::xxh3_128(input.as_slice()));
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("Twox/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            black_box(twox_hash::XxHash3_128::oneshot(input.as_slice()));
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("C/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            unsafe {
                xxhash_c_sys::XXH3_128bits(input.as_ptr() as _, input.len());
            }
        }, criterion::BatchSize::SmallInput));
    }

    pub fn xxh3_seedless_stream64<const SIZE: usize>(group: &mut BenchmarkGroup<'_, impl Measurement>, data: &[[u8; SIZE]]) {
        group.throughput(Throughput::Bytes(SIZE as _));
        group.bench_function(&format!("Rust/{SIZE}b"), move |b| b.iter_batched(move || data, |data| for input in data {
            let mut hasher = xxhash_rust::xxh3::Xxh3::new();
            hasher.update(black_box(input.as_slice()));
            black_box(hasher.digest());
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("Twox/{SIZE}b"), move |b| b.iter_batched(move || data, |data| for input in data {
            use core::hash::Hasher;

            let mut hasher = twox_hash::XxHash3_64::new();
            hasher.write(black_box(input.as_slice()));
            black_box(hasher.finish());
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("RustDefault/{SIZE}b"), move |b| b.iter_batched(move || data, |data| for input in data {
            let mut hasher = xxhash_rust::xxh3::Xxh3Default::new();
            hasher.update(black_box(input.as_slice()));
            black_box(hasher.digest());
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("C/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            use xxhash_c_sys as sys;

            let mut state = core::mem::MaybeUninit::<sys::XXH3_state_t>::uninit();

            unsafe {
                sys::XXH3_64bits_reset(state.as_mut_ptr());
                sys::XXH3_64bits_update(state.as_mut_ptr(), input.as_ptr() as _, input.len());
                sys::XXH3_64bits_digest(state.as_mut_ptr());
            }
        }, criterion::BatchSize::SmallInput));
    }

    pub fn xxh3_seedless_stream128<const SIZE: usize>(group: &mut BenchmarkGroup<'_, impl Measurement>, data: &[[u8; SIZE]]) {
        group.throughput(Throughput::Bytes(SIZE as _));
        group.bench_function(&format!("Rust/{SIZE}b"), move |b| b.iter_batched(move || data, |data| for input in data {
            let mut hasher = xxhash_rust::xxh3::Xxh3::new();
            hasher.update(black_box(input.as_slice()));
            black_box(hasher.digest128());
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("Twox/{SIZE}b"), move |b| b.iter_batched(move || data, |data| for input in data {
            let mut hasher = twox_hash::XxHash3_128::new();
            hasher.write(black_box(input.as_slice()));
            black_box(hasher.finish_128());
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("RustDefault/{SIZE}b"), move |b| b.iter_batched(move || data, |data| for input in data {
            let mut hasher = xxhash_rust::xxh3::Xxh3Default::new();
            hasher.update(black_box(input.as_slice()));
            black_box(hasher.digest128());
        }, criterion::BatchSize::SmallInput));
        group.bench_function(&format!("C/{SIZE}b"), |b| b.iter_batched(|| data, |data| for input in data {
            use xxhash_c_sys as sys;

            let mut state = core::mem::MaybeUninit::<sys::XXH3_state_t>::uninit();

            unsafe {
                sys::XXH3_128bits_reset(state.as_mut_ptr());
                sys::XXH3_128bits_update(state.as_mut_ptr(), input.as_ptr() as _, input.len());
                sys::XXH3_128bits_digest(state.as_mut_ptr());
            }
        }, criterion::BatchSize::SmallInput));
    }
}

fn main() {
  let mut criterion = Criterion::default().configure_from_args();

  macro_rules! run_group {
      ($group:ident) => {
          runners::$group(&mut $group, DATA_3);
          runners::$group(&mut $group, DATA_6);
          runners::$group(&mut $group, DATA_9);
          runners::$group(&mut $group, DATA_16);
          runners::$group(&mut $group, DATA_17);
          runners::$group(&mut $group, DATA_64);
          runners::$group(&mut $group, DATA_128);
          runners::$group(&mut $group, DATA_149);
          runners::$group(&mut $group, DATA_240);
          runners::$group(&mut $group, DATA_2046);

      };
  }
  let mut xxh3_seedless_oneshot64 = criterion.benchmark_group("xxh3/oneshot/64");
  run_group!(xxh3_seedless_oneshot64);
  xxh3_seedless_oneshot64.finish();
  let mut xxh3_seedless_oneshot128 = criterion.benchmark_group("xxh3/oneshot/128");
  run_group!(xxh3_seedless_oneshot128);
  xxh3_seedless_oneshot128.finish();
  let mut xxh3_seedless_stream64 = criterion.benchmark_group("xxh3/stream/64");
  run_group!(xxh3_seedless_stream64);
  xxh3_seedless_stream64.finish();
  let mut xxh3_seedless_stream128 = criterion.benchmark_group("xxh3/stream/128");
  run_group!(xxh3_seedless_stream128);
  xxh3_seedless_stream128.finish();

  Criterion::default().configure_from_args().final_summary();
}
