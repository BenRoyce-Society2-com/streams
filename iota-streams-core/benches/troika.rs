#[macro_use]
extern crate criterion;

use criterion::{Benchmark, Criterion};
use iota_streams_core::sponge::{prp::troika::Troika, spongos::Spongos};
use iota_streams_core::tbits::{trinary::Trit, Tbits};
use std::time::Duration;

fn step(key: &Tbits<Trit>, x: &Tbits<Trit>) {
    const MAC_SIZE: usize = Spongos::<Trit, Troika>::MAC_SIZE;
    let mut s = Spongos::<Trit, Troika>::init();
    s.absorb_tbits(key);
    s.absorb_tbits(x);
    s.commit();
    s.encrypt_tbits(x);
    s.commit();
    s.squeeze_tbits(MAC_SIZE);
}

fn keccakf1600t_benchmark(c: &mut Criterion) {
    const KEY_SIZE: usize = Spongos::<Trit, Troika>::KEY_SIZE;

    {
        let key = Tbits::<Trit>::zero(KEY_SIZE);
        let x5T = Tbits::<Trit>::zero(5);
        c.bench_function("Run Troika spongos (5T)", move |b| {
            b.iter(|| step(&key, &x5T))
        });
    }

    {
        let key = Tbits::<Trit>::zero(KEY_SIZE);
        let x5KiT = Tbits::<Trit>::zero(5 * 1024);
        c.bench_function("Run Troika spongos (5KiT)", move |b| {
            b.iter(|| {
                step(&key, &x5KiT);
            })
        });
    }

    {
        let key = Tbits::<Trit>::zero(KEY_SIZE);
        let x5MiT = Tbits::<Trit>::zero(5 * 1024 * 1024);
        c.bench(
            "Run Troika spongos",
            Benchmark::new(" (5MiT)", move |b| {
                b.iter(|| {
                    step(&key, &x5MiT);
                })
            })
            .sample_size(10)
            .measurement_time(Duration::from_millis(10000)),
        );
    }
}

criterion_group!(benches, keccakf1600t_benchmark);
criterion_main!(benches);
