use criterion::{black_box, criterion_group, criterion_main, Criterion};
use keysymdefs::{
    get_item_by_cleared_name, get_item_by_keysym, get_item_by_name, get_item_by_unicode,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("#get_item_by_keysym", |b| {
        b.iter(|| get_item_by_keysym(black_box(269025044)))
    });
    c.bench_function("#get_item_by_unicode", |b| {
        b.iter(|| get_item_by_unicode(black_box(3550)))
    });
    c.bench_function("#get_item_by_name", |b| {
        b.iter(|| get_item_by_name(black_box("XF86XK_MonBrightnessUp")))
    });
    c.bench_function("#get_item_by_cleared_name", |b| {
        b.iter(|| get_item_by_cleared_name(black_box("KbdLightOnOff")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
