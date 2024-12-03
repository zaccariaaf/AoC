fn main() {
    println!("Day 1, Part 1: {}", d1_p1());
}

fn d1_p1() -> u32 {
    let mut x: Vec<u32> = Vec::new();
    let mut y: Vec<u32> = Vec::new();
    include_str!("../resources/input_day1.txt")
        .lines()
        .for_each(|line| {
            line.split_whitespace()
                .zip(line.split_whitespace().skip(1))
                .for_each(|(a, b)| {
                    x.push(a.parse().unwrap());
                    y.push(b.parse().unwrap());
                });
        });
    x.sort();
    y.sort();
    x.iter().zip(y.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}
