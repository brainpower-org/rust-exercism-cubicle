#![feature(test)]
extern crate bob;
extern crate test;

use test::Bencher;

#[bench]
fn bench_lorem_collect(b: &mut Bencher) {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
    b.iter(|| bob::shouting(lorem));
}

#[bench]
fn bench_lorem_peek(b: &mut Bencher) {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
    b.iter(|| bob::shouting_peek(lorem));
}

#[bench]
fn bench_lorem_imperative(b: &mut Bencher) {
    let lorem = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
    b.iter(|| bob::shouting_imperative(lorem));
}

#[bench]
fn bench_a_collect(b: &mut Bencher) {
    b.iter(|| bob::shouting("A"));
}

#[bench]
fn bench_a_peek(b: &mut Bencher) {
    b.iter(|| bob::shouting_peek("A"));
}

#[bench]
fn bench_a_imperative(b: &mut Bencher) {
    b.iter(|| bob::shouting_imperative("A"));
}