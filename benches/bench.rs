/*
comment out all the #[wasm_bindgen] annotations,
and the "cdylib" bits from Cargo.toml

$ cargo bench | tee before.txt

https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
require "cargo"
$ rustup toolchain install nightly
$ rustup toolchain list
$ rustup override set nightly
$ cargo install cargo-benchcmp

install perf in ubuntu
$ sudo perf record -g  target/release/deps/bench-63c7b5835f43dd49 --bench
$ perf report
 */
#![feature(test)]

extern crate test;
extern crate wasm_game_of_life;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = wasm_game_of_life::Universe::new();

    b.iter(|| universe.tick());
}