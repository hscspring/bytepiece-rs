use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use lazy_static::lazy_static;

use bytepiece_rs::Tokenizer;
use bytepiece_rs::read_to_string;


lazy_static! {
    static ref TEXT: String = read_to_string("bench_aho/data/鲁迅全集.txt");
    static ref BYTEPIECE: Tokenizer = Tokenizer::new();
}


fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("bytepiece_diff_len_alpha");
    for size in [100, 1000, 10000, 100000].iter() {
        let text = &TEXT.chars().take(*size).collect::<String>();
        for alpha in [-1.0, 0.1].iter() {
            group.throughput(Throughput::Bytes(*size as u64));
            group.sample_size(10);
            group.bench_with_input(
                format!("size={}, alpha={}", size, alpha), &(text, alpha), 
                |b, (text, &alpha)| {
                b.iter(|| {
                    let _ids = BYTEPIECE.encode(
                        &text, false, false, alpha, true
                    );
                })
            });
        }
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);