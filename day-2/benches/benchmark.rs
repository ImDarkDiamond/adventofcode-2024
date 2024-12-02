use day_2::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::run(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::run(divan::black_box(include_str!("../input1.txt",))).unwrap();
}
