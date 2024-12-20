use day_10::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}

#[divan::bench]
fn part1_rayon() {
    part1_rayon::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part2_rayon() {
    part2_rayon::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}
