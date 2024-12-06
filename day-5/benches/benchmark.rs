use day_5::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::run(divan::black_box(include_str!("../input1.txt",))).unwrap();
}