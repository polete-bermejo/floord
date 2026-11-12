use criterion::{Criterion, criterion_group, criterion_main};
use floord::{cleanup_service, saving_service};
use std::fs::{create_dir, create_dir_all, remove_dir_all};
use std::time::Duration;
use tempfile::tempdir_in;

fn floord_empty_benchmark(c: &mut Criterion) {
    let tmp = tempdir_in(".").unwrap();
    let tmp = tmp.path();

    let secs = Duration::new(0, 0);

    c.bench_function("empty folder", |b| b.iter(|| saving_service(tmp, &secs)));

    remove_dir_all(tmp).unwrap()
}

fn floord_10k_benchmark(c: &mut Criterion) {
    let tmp = tempdir_in(".").unwrap();
    let tmp = tmp.path();
    let secs = Duration::new(300, 0);
    let max = 10_000;

    for i in 0..max {
        create_dir(tmp.join(i.to_string())).unwrap()
    }

    c.bench_function("10k not moving", |b| b.iter(|| saving_service(tmp, &secs)));

    remove_dir_all(tmp).unwrap()
}

fn floord_500_move_dirs_benchmark(c: &mut Criterion) {
    let tmp = tempdir_in(".").unwrap();
    let tmp = tmp.path();
    let secs = Duration::ZERO;
    let max = 500;

    c.bench_function("500 moving dirs", |b| {
        b.iter_batched(
            || {
                for i in 0..max {
                    create_dir(tmp.join(i.to_string())).unwrap();
                }

                tmp.to_path_buf()
            },
            |path| {
                saving_service(&path, &secs);
            },
            criterion::BatchSize::LargeInput,
        )
    });

    remove_dir_all(tmp).unwrap();
}

fn cleanup_empty_dir(c: &mut Criterion) {
    let tmp = tempdir_in(".").unwrap();
    let path = tmp.path();
    let duration = Duration::ZERO;
    c.bench_function("cleanup empty dir", |b| {
        b.iter(|| {
            cleanup_service(&vec![path], &duration, &duration);
        })
    });
}

fn cleanup_10k_dir_not_moving(c: &mut Criterion) {
    let tmp = tempdir_in(".").unwrap();
    let path = tmp.path();
    let duration = Duration::ZERO;
    let size = 10_000;
    for i in 0..size {
        create_dir_all(path.join(format!("asdf{i}"))).unwrap();
    }
    c.bench_function("cleanup empty dir", |b| {
        b.iter(|| {
            cleanup_service(&vec![path], &duration, &duration);
        })
    });
}

criterion_group!(
    benches,
    floord_empty_benchmark,
    floord_10k_benchmark,
    cleanup_empty_dir,
    cleanup_10k_dir_not_moving,
    floord_500_move_dirs_benchmark,
);
criterion_main!(benches);
