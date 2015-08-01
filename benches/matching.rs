#![feature(test)]

extern crate test;
extern crate bloodhound;

use test::Bencher;
use std::path::PathBuf;
use bloodhound::entry;
use bloodhound::matching::find;

#[bench]
fn bench_find(b: &mut Bencher) {
    let haystack = vec![
        entry::new("src/hound.rs".to_string()),
        entry::new("lib/hounds.rs".to_string()),
        entry::new("Houndfile".to_string())
    ];
    b.iter(|| find("match", &haystack, 5));
}

#[bench]
fn bench_similarity(b: &mut Bencher) {
    let entry = entry::new("bloodhound/src/matching.rs".to_string());
    b.iter(|| entry.similarity("matching.rs"));
}
