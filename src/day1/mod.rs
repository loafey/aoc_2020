use aoc_lib::load_to_rows;

pub fn part1() -> i32 {
    let i = load_to_rows("input/day1.txt")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for a in &i {
        for b in &i {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    0
}
pub fn part2() -> i32 {
    let i = load_to_rows("input/day1.txt")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for a in &i {
        for b in &i {
            for c in &i {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    0
}
