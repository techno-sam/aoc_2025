use criterion::criterion_main;

bench_macros::setup_up_to!(8);
criterion_main!(days);
