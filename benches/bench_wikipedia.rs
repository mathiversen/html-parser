use criterion::{criterion_group, criterion_main, Criterion};
use html_parser::Dom;

static HTML: &'static str = include_str!("./wikipedia-2020-12-21.html");

fn wikipedia(c: &mut Criterion) {
    c.bench_function("wikipedia", |b| b.iter(|| Dom::parse(HTML).unwrap()));
}

criterion_group!(benches, wikipedia);
criterion_main!(benches);
