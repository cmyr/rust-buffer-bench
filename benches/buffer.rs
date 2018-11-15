
#[macro_use]
extern crate criterion;
extern crate buffer_bench;
extern crate xi_rope;
extern crate xi_rope_master;
extern crate ropey;


use std::iter;
use criterion::Criterion;

use xi_rope::rope::Rope;
use xi_rope_master::Rope as Rope3;
use ropey::Rope as RopeyRope;
use buffer_bench::Buffer;

fn make_string(len: usize) -> String {
    iter::repeat(' ').take(len).collect::<String>()
}

fn basic_append<B: Buffer>() {
    let mut b = B::new("");
    for _ in 0..1000 {
        b.append("a");
    }
}

fn big_append<B: Buffer>() {
    let mut b = B::new("");
    for _ in 0..50_000 {
        b.append("a");
    }
}

fn basic_insert<B: Buffer>() {
    let mut b = B::new("AA");
    for i in 0..1000 {
        b.insert(1, "B");
    }
}

fn big_insert<B: Buffer>() {
    let mut b = B::new(&make_string(10_000));
    for i in 0..1000 {
        b.insert(4000, "A");
    }
}

fn rope_append(c: &mut Criterion) {
    c.bench_function("rope_append", |b| b.iter(|| basic_append::<Rope>()));
}

fn rope3_append(c: &mut Criterion) {
    c.bench_function("rope3_append", |b| b.iter(|| basic_append::<Rope3>()));
}

fn ropey_append(c: &mut Criterion) {
    c.bench_function("ropey_append", |b| b.iter(|| basic_append::<RopeyRope>()));
}

fn string_append(c: &mut Criterion) {
    c.bench_function("string_append", |b| b.iter(|| basic_append::<String>()));
}

// fn rope_big_append(c: &mut Criterion) {
//     c.bench_function("rope_big_append", |b| b.iter(|| big_append::<Rope>()));
// }
// 
// fn string_big_append(c: &mut Criterion) {
//     c.bench_function("string_big_append", |b| b.iter(|| big_append::<String>()));
// }

fn rope_insert(c: &mut Criterion) {
    c.bench_function("rope_insert", |b| b.iter(|| basic_insert::<Rope>()));
}

fn rope3_insert(c: &mut Criterion) {
    c.bench_function("rope3_insert", |b| b.iter(|| basic_insert::<Rope3>()));
}

fn ropey_insert(c: &mut Criterion) {
    c.bench_function("ropey_insert", |b| b.iter(|| basic_insert::<RopeyRope>()));
}

fn string_insert(c: &mut Criterion) {
    c.bench_function("string_insert", |b| b.iter(|| basic_insert::<String>()));
}

fn rope_big_insert(c: &mut Criterion) {
    c.bench_function("rope_big_insert", |b| b.iter(|| big_insert::<Rope>()));
}

fn rope3_big_insert(c: &mut Criterion) {
    c.bench_function("rope3_big_insert", |b| b.iter(|| big_insert::<Rope3>()));
}

fn ropey_big_insert(c: &mut Criterion) {
    c.bench_function("ropey_big_insert", |b| b.iter(|| big_insert::<RopeyRope>()));
}

fn string_big_insert(c: &mut Criterion) {
    c.bench_function("string_big_insert", |b| b.iter(|| big_insert::<String>()));
}

criterion_group!(append, string_append, rope_append, rope3_append, ropey_append);
criterion_group!(insert, string_insert, rope_insert, rope3_insert, ropey_insert);
criterion_group!(insert_big, string_big_insert, rope_big_insert, rope3_big_insert, ropey_big_insert);
criterion_main!(append, insert, insert_big);
// criterion_main!(insert);