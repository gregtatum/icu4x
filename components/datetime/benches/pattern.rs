mod fixtures;

use criterion::{criterion_group, criterion_main, Criterion};

use icu_datetime::pattern::Pattern;

fn pattern_benches(c: &mut Criterion) {
    let patterns: Vec<String> = fixtures::get_patterns_fixture().unwrap().0;

    {
        let mut group = c.benchmark_group("pattern");

        group.bench_function("parse", |b| {
            b.iter(|| {
                for input in &patterns {
                    let _ = Pattern::from_bytes(input).unwrap();
                }
            })
        });

        group.finish();
    }
}

criterion_group!(benches, pattern_benches,);
criterion_main!(benches);
