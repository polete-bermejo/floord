use criterion::{Criterion, criterion_group, criterion_main};
use floord::saving_service;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use std::time::Duration;

const TEST_PATH: &str = "/tmp/floord";

fn floord_empty_benchmark(c: &mut Criterion) {
    let tmp = Path::new(TEST_PATH);
    remove_dir_all(tmp).unwrap_or(());
    create_dir(TEST_PATH).unwrap_or(());

    let secs = Duration::new(0, 0);

    c.bench_function("empty folder", |b| b.iter(|| saving_service(tmp, &secs)));

    remove_dir_all(tmp).unwrap()
}

fn floord_10k_benchmark(c: &mut Criterion) {
    create_dir(TEST_PATH).unwrap_or(());

    let tmp = Path::new(TEST_PATH);
    let secs = Duration::new(300, 0);
    let max = 10_000;

    for i in 0..max {
        create_dir(tmp.join(i.to_string())).unwrap()
    }

    c.bench_function("10k not moving", |b| b.iter(|| saving_service(tmp, &secs)));

    remove_dir_all(tmp).unwrap()
}

fn floord_500_move_dirs_benchmark(c: &mut Criterion) {
    let tmp = Path::new(TEST_PATH);
    let secs = Duration::ZERO;
    let max = 500;

    c.bench_function("500 moving dirs", |b| {
        b.iter_batched(
            || {
                create_dir(TEST_PATH).unwrap_or(());

                for i in 0..max {
                    create_dir(tmp.join(i.to_string())).unwrap();
                }

                tmp.to_path_buf()
            },
            |path| {
                saving_service(&path, &secs).unwrap();
            },
            criterion::BatchSize::LargeInput,
        )
    });

    remove_dir_all(tmp).unwrap();
}

criterion_group!(
    benches,
    floord_empty_benchmark,
    floord_10k_benchmark,
    floord_500_move_dirs_benchmark
);
criterion_main!(benches);
