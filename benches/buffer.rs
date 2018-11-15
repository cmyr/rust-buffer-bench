
#[macro_use]
extern crate criterion;
extern crate buffer_bench;
extern crate xi_rope;
extern crate xi_rope_master;
extern crate xi_rope_rc;
extern crate ropey;



use std::iter;
use criterion::{Criterion, Fun};

use xi_rope::rope::Rope as Ropev2;
use xi_rope_master::Rope as Ropev3;
use xi_rope_rc::Rope as RopeRc;
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

fn basic_appends(c: &mut Criterion) {
    let string = Fun::new("std::String", |b, _| b.iter(|| basic_append::<String>()));
    let ropev02 = Fun::new("xi_rope_0.2::Rope", |b, _| b.iter(|| basic_append::<Ropev2>()));
    let ropev03 = Fun::new("xi_rope_0.3::Rope", |b, _| b.iter(|| basic_append::<Ropev3>()));
    let roperc = Fun::new("xi_rope_rc::Rope", |b, _| b.iter(|| basic_append::<RopeRc>()));
    let ropey = Fun::new("ropey::Rope", |b, _| b.iter(|| basic_append::<RopeyRope>()));
    let functions = vec![string, ropev02, ropev03, roperc, ropey];
    c.bench_functions("basic_appends", functions, ());
}

// fn big_append<B: Buffer>() {
//     let mut b = B::new("");
//     for _ in 0..50_000 {
//         b.append("a");
//     }
// }

fn basic_insert<B: Buffer>() {
    let mut b = B::new("AAAA");
    for i in 1..=1000 {
        b.insert(i, "B");
    }
}

fn basic_inserts(c: &mut Criterion) {
    let string = Fun::new("std::String", |b, _| b.iter(|| basic_insert::<String>()));
    let ropev02 = Fun::new("xi_rope_0.2::Rope", |b, _| b.iter(|| basic_insert::<Ropev2>()));
    let ropev03 = Fun::new("xi_rope_0.3::Rope", |b, _| b.iter(|| basic_insert::<Ropev3>()));
    let roperc = Fun::new("xi_rope_rc::Rope", |b, _| b.iter(|| basic_insert::<RopeRc>()));
    let ropey = Fun::new("ropey::Rope", |b, _| b.iter(|| basic_insert::<RopeyRope>()));
    let functions = vec![string, ropev02, ropev03, roperc, ropey];
    c.bench_functions("basic_inserts", functions, ());
}

fn insert_into_big<B: Buffer>() {
    let mut b = B::new(&make_string(1_000_000));
    for i in 1..=100 {
        let idx = i * 900;
        b.insert(idx, "A");
    }
}

fn inserts_into_big(c: &mut Criterion) {
    let string = Fun::new("std::String", |b, _| b.iter(|| insert_into_big::<String>()));
    let ropev02 = Fun::new("xi_rope_0.2::Rope", |b, _| b.iter(|| insert_into_big::<Ropev2>()));
    let ropev03 = Fun::new("xi_rope_0.3::Rope", |b, _| b.iter(|| insert_into_big::<Ropev3>()));
    let ropey = Fun::new("ropey::Rope", |b, _| b.iter(|| insert_into_big::<RopeyRope>()));
    let roperc = Fun::new("xi_rope_rc::Rope", |b, _| b.iter(|| insert_into_big::<RopeRc>()));
    let functions = vec![string, ropev02, ropev03, roperc, ropey];
    c.bench_functions("insert_into_big", functions, ());
}

criterion_group!(basic_inserts_group, basic_inserts);
criterion_group!(inserts_into_big_group, inserts_into_big);
criterion_group!(basic_appends_group, basic_appends);
criterion_main!(basic_appends_group, basic_inserts_group, inserts_into_big_group);