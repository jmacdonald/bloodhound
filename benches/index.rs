#![feature(test)]

extern crate test;
extern crate bloodhound;

use test::Bencher;
use std::path::PathBuf;
use bloodhound::Index;

#[bench]
fn bench_find(b: &mut Bencher) {
    let mut index = Index::new(PathBuf::from("."));
    index.populate(None, false);
    b.iter(|| index.find("match", 5));
}
