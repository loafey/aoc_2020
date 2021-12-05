use aoc_lib::load_to_rows;

pub fn part1() -> usize {
    load_to_rows("input/day2.txt")
        .filter(|s| !s.is_empty())
        .filter(|s| {
            let mut s = s.split(' ');
            let (a, b) = {
                let mut nums = s.next().unwrap().split('-');
                (
                    nums.next().unwrap().parse::<usize>().unwrap(),
                    nums.next().unwrap().parse::<usize>().unwrap(),
                )
            };
            let letter = s.next().unwrap().chars().next().unwrap();
            let pass = s.next().unwrap();

            let count = pass.chars().filter(|c| *c == letter).count();
            a <= count && count <= b
        })
        .count()
}
pub fn part2() -> usize {
    load_to_rows("input/day2.txt")
        .filter(|s| !s.is_empty())
        .filter(|s| {
            let mut s = s.split(' ');
            let (a, b) = {
                let mut nums = s.next().unwrap().split('-');
                (
                    nums.next().unwrap().parse::<usize>().unwrap() - 1,
                    nums.next().unwrap().parse::<usize>().unwrap() - 1,
                )
            };
            let letter = s.next().unwrap().chars().next().unwrap();
            let pass = s.next().unwrap();

            let a = pass.chars().nth(a).unwrap();
            let b = pass.chars().nth(b).unwrap();
            (a == letter && b != letter) || (a != letter && b == letter)
        })
        .count()
}
