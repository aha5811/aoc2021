mod util;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    println!("day01 #1: {}", day01::part1("day01"));
    util::test_i32("day01 #2 test", 5, day01::part2("day01_2_test"));
    println!("day01 #2: {}", day01::part2("day01"));
    println!("-------------------------------------");
    println!("day02 #1: {}", day02::part1("day02"));
    util::test_i32("day02 #2 test", 900, day02::part2("day02_2_test"));
    println!("day02 #2: {}", day02::part2("day02"));
    println!("-------------------------------------");
    println!("day03 #1: {}", day03::part1("day03"));
    util::test_i32("day03 #2 test", 230, day03::part2("day03_2_test"));
    println!("day03 #2: {}", day03::part2("day03"));
    println!("-------------------------------------");
    util::test_i32("day04 #1 test", 4512, day04::part1("day04_test"));
    println!("day04 #1: {}", day04::part1("day04"));
    util::test_i32("day04 #2 test", 1924, day04::part2("day04_test"));
    println!("day04 #2: {}", day04::part2("day04"));
    println!("-------------------------------------");
    util::test_i32("day05 #1 test", 5, day05::part1("day05_test"));
    println!("day05 #1: {}", day05::part1("day05"));
    util::test_i32("day05 #2 test", 12, day05::part2("day05_test"));
    println!("day05 #2: {}", day05::part2("day05"));
    println!("-------------------------------------");
    util::test_i32("day06 #1 test", 26, day06::part1("day06_test", 18));
    util::test_i32("day06 #1 test", 5934, day06::part1("day06_test", 80));
    println!("day06 #1: {}", day06::part1("day06", 80));
    util::test_i128("day06 #2 test", 26984457539, day06::part2("day06_test", 256));
    println!("day06 #2: {}", day06::part2("day06", 256));
    println!("-------------------------------------");
}
