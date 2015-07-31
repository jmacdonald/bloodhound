#![feature(test)]

extern crate test;
extern crate bloodhound;

use test::Bencher;
use std::path::PathBuf;
use bloodhound::matching::{find, similarity};

#[bench]
fn bench_find(b: &mut Bencher) {
    let haystack = vec![PathBuf::from("src/hound.rs"),
        PathBuf::from("lib/hounds.rs"), PathBuf::from("Houndfile")];
    b.iter(|| find("match", &haystack, 5));
}

#[bench]
fn bench_similarity(b: &mut Bencher) {
    b.iter(|| similarity("matching.rs", "bloodhound/src/matching.rs"));
}
