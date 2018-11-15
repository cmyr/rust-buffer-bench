
#[macro_use]
extern crate criterion;
extern crate buffer_bench;
extern crate xi_rope;
extern crate xi_rope_master;
extern crate ropey;


use std::iter;
use criterion::{Criterion, Fun};

use xi_rope::rope::Rope as Ropev2;
use xi_rope_master::Rope as Ropev3;
use ropey::Rope as RopeyRope;
use buffer_bench::Buffer;

// fn make_string(len: usize) -> String {
//     iter::repeat(' ').take(len).collect::<String>()
// }

fn basic_append<B: Buffer>() {
    let mut b = B::new("");
    for _ in 0..1000 {
        b.append("a");
    }
}

fn basic_appends(c: &mut Criterion) {
    let string = Fun::new("std::String", |b, _| b.iter(|| basic_append::<String>()));
    let ropev02 = Fun::new("xi_rope_0.2::Rope", |b, _| b.iter(|| basic_append::<Ropev2>()));
    let ropev03 = Fun::new("xi_rope_0.3::Rope", |b, _| b.iter(|| basic_append::<Ropev3>()));
    let ropey = Fun::new("ropey::Rope", |b, _| b.iter(|| basic_append::<RopeyRope>()));
    let functions = vec![string, ropev02, ropev03, ropey];
    c.bench_functions("basic_appends", functions, ());
}

// fn big_append<B: Buffer>() {
//     let mut b = B::new("");
//     for _ in 0..50_000 {
//         b.append("a");
//     }
// }

fn basic_insert<B: Buffer>() {
    let mut b = B::new("AA");
    for i in 0..1000 {
        b.insert(1, "B");
        b.insert(1, "B");
    }
}

fn basic_inserts(c: &mut Criterion) {
    let string = Fun::new("std::String", |b, _| b.iter(|| basic_insert::<String>()));
    let ropev02 = Fun::new("xi_rope_0.2::Rope", |b, _| b.iter(|| basic_insert::<Ropev2>()));
    let ropev03 = Fun::new("xi_rope_0.3::Rope", |b, _| b.iter(|| basic_insert::<Ropev3>()));
    let ropey = Fun::new("ropey::Rope", |b, _| b.iter(|| basic_insert::<RopeyRope>()));
    let functions = vec![string, ropev02, ropev03, ropey];
    c.bench_functions("basic_inserts", functions, ());
}

// fn big_insert<B: Buffer>() {
//     let mut b = B::new(&make_string(10_000));
//     for i in 0..1000 {
//         b.insert(4000, "A");
//         b.insert(4000, "A");
//     }
// }

criterion_group!(basic_inserts_group, basic_inserts);
criterion_group!(basic_appends_group, basic_appends);
criterion_main!(basic_appends_group);