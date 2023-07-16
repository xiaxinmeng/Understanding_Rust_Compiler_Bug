rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // don't mind the naming of the benchmarks
    let mut group = c.benchmark_group("dark theme");
    // the string literal being the text of the file
    group.bench_function("new", |b| b.iter(|| new(black_box([string literal].to_string()))));
    group.finish();
}

fn old(s: String) -> String {
    s
        .trim()
        .replace('\n', " ")
        .replace('\t', " ")
        .replace('/', "")
        .replace('{', "")
        .replace('}', "")
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

fn new(s: String) -> String {
    s
        .trim()
        .chars()
        .filter_map(|c| match c {
            '\n' | '\t' => Some(' '),
            '/' | '{' | '}' => None,
            c => Some(c),
        })
        .collect::<String>()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
