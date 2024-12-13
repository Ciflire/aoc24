use std::time::Instant;

use day1::day1_1;
use day1::day1_2;
use day2::day2_1;
use day2::day2_2;
use day3::day3_1;
use day3::day3_2;
use day4::day4_1;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    // Day 1
    let now_part1 = Instant::now();
    day1_1();
    let time_to_run_1_1 = now_part1.elapsed();
    let now_part2 = Instant::now();
    day1_2();
    let time_to_run_1_2 = now_part2.elapsed();
    println!("Day 1: {:?} : {:?}", time_to_run_1_1, time_to_run_1_2);

    // Day 2
    let now_part1 = Instant::now();
    day2_1();
    let time_to_run_2_1 = now_part1.elapsed();
    let now_part2 = Instant::now();
    day2_1();
    let time_to_run_2_2 = now_part2.elapsed();
    println!("Day 2 : {:?} : {:?}", time_to_run_2_1, time_to_run_2_2);

    // Day 3
    let now_part1 = Instant::now();
    day3_1();
    let time_to_run_3_1 = now_part1.elapsed();
    let now_part2 = Instant::now();
    day3_2();
    let time_to_run_3_2 = now_part2.elapsed();
    println!("Day 3 : {:?} : {:?}", time_to_run_3_1, time_to_run_3_2);

    // Day 4
    let now_part1 = Instant::now();
    day4_1();
    let time_to_run_4_1 = now_part1.elapsed();
    // let now_part2 = Instant::now();
    // day4_2();
    // let time_to_run_4_2 = now_part2.elapsed();
    println!("Day 4 : {:?} :", time_to_run_4_1 /*time_to_run_4_2*/,);
}
