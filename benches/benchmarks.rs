#[macro_use]
extern crate criterion;

use criterion::Criterion;
use nix::unistd::getcwd;
use std::env::current_dir;
use std::path::PathBuf;

fn get_cwd_from_env() -> PathBuf {
    return current_dir().unwrap();
}

fn get_cwd_unistd() -> PathBuf {
    return getcwd().unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cwd_from_env", |b| b.iter(|| get_cwd_from_env()));
    c.bench_function("cwd_unistd", |b| b.iter(|| get_cwd_unistd()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
