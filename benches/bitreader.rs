use brotschneider::bitreader::BitReader;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

pub fn bitreader_is_empty_benchmark(c: &mut Criterion) {
    let data = vec![0u8; 1024];

    c.bench_function("BitReader::is_empty()", |b| {
        b.iter(|| {
            let reader = BitReader::new(black_box(&data));
            black_box(reader.is_empty());
        });
    });
}

criterion_group!(benches, bitreader_is_empty_benchmark);
criterion_main!(benches);
