use criterion::{criterion_group, criterion_main, Criterion};
use huff_tree_tap::HuffmanData;
use std::hint::black_box;

fn huffman_encode_bench(input_data: &[u8]) -> HuffmanData {
    HuffmanData::new(input_data).unwrap()
}

fn huffman_decode_bench(input_data: &HuffmanData) -> Vec<u8> {
    input_data.decode().unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut unencoded_data = Vec::<u8>::new();
    for _ in 0..1000 {
        unencoded_data.append(&mut vec![b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h']);
    }

    let encoded_data = HuffmanData::new(&unencoded_data).unwrap();

    let mut group = c.benchmark_group("Huffman");
    group.bench_function("huffman_encode", |b| {
        b.iter(|| black_box(huffman_encode_bench(black_box(&unencoded_data))))
    });
    group.bench_function("huffman_decode", |b| {
        b.iter(|| black_box(huffman_decode_bench(black_box(&encoded_data))))
    });
    group.finish();
}

// Encode: 650micros Decode: 1.14ms
criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(std::time::Duration::from_secs(10));
    targets = criterion_benchmark
);
criterion_main!(benches);
