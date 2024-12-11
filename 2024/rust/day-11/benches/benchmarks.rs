use day_11::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input1.txt",))).unwrap();
}

#[divan::bench]
fn part1_recursive() {
    part1_recursive::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}

#[divan::bench]
fn part1_recursive_cached() {
    part1_recursive_cached::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}

#[divan::bench]
fn part1_recursive_cached_par() {
    part1_recursive_cached_par::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}

// DNF
// #[divan::bench]
// fn part2() {
//     part2::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
// }

// DNF
// #[divan::bench]
// fn part2_recursive() {
//     part2_recursive::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
// }

#[divan::bench]
fn part2_recursive_cached() {
    part2_recursive_cached::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}

#[divan::bench]
fn part2_recursive_cached_par() {
    part2_recursive_cached_par::process(divan::black_box(include_str!("../input2.txt",))).unwrap();
}
