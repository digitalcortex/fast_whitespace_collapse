use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use regex::Regex;

fn regex_approach(s: &str, re: &Regex) -> String {
    re.replace_all(s, " ").to_string()
}

fn iterative(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut last_was_space = true; // Start true to trim leading spaces
    
    for c in s.chars() {
        if c == ' ' || c == '\t' {
            if !last_was_space {
                result.push(' ');
                last_was_space = true;
            }
        } else {
            result.push(c);
            last_was_space = false;
        }
    }
    
    // Trim trailing space if exists
    if result.ends_with(' ') {
        result.pop();
    }
    
    result
}

fn iterative_bytes(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut result = Vec::with_capacity(bytes.len());
    
    let mut last_was_space = true;

    for &b in bytes {
        if b == b' ' || b == b'\t' {
            if !last_was_space {
                result.push(b' ');
                last_was_space = true;
            }
        } else {
            result.push(b);
            last_was_space = false;
        }
    }

    // Trim trailing space if exists
    if result.last() == Some(&b' ') {
        result.pop();
    }

    // Convert back to a String
    unsafe { String::from_utf8_unchecked(result) }
}

fn cargo_collapse(s: &str) -> String {
    collapse::collapse(s).to_string()
}

fn benchmark(c: &mut Criterion) {
    let s = r#"
        This  is    a   test   text   with    irregular spacing.

        It    contains     multiple    spaces,    
        tabs,	and		newlines.


        The    goal     is     to     check     
        how     whitespace      collapses.

        Some    lines      contain      extra       spaces.

            Some       have       leading       spaces.

        Some have trailing spaces.     

        Some have  both.    

        Whitespace       collapse     testing.
    "#;

    let mut group = c.benchmark_group("whitespace_normalization");
    group.sample_size(500);
    group.measurement_time(Duration::from_secs(20));

    group.bench_function("regex_approach", |b| {
        b.iter_batched(|| (s, Regex::new(r"\s\s+").unwrap()), |(s, re)| regex_approach(black_box(&s), black_box(&re)), criterion::BatchSize::LargeInput)
    });

    group.bench_function("iterative_approach", |b| {
        b.iter_batched(|| s, |s| iterative(black_box(s)), criterion::BatchSize::LargeInput)
    });

    group.bench_function("iterative_bytes", |b| {
        b.iter_batched(|| s, |s| iterative_bytes(black_box(s)), criterion::BatchSize::LargeInput)
    });

    group.bench_function("collapse_library", |b| {
        b.iter_batched(|| s, |s| cargo_collapse(black_box(s)), criterion::BatchSize::LargeInput)
    });

    group.bench_function("fast_whitespace_collapse", |b| {
        b.iter_batched(|| s, |s| fast_whitespace_collapse::collapse_whitespace(black_box(s)), criterion::BatchSize::LargeInput)
    });

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
