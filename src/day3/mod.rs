use aoc_lib::load_to_matrix;

fn sloaper(m: &[Vec<char>], delta_x: usize, delta_y: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let mut trees = 0;
    while y < m.len() - 1 {
        if m[y][x] == '#' {
            trees += 1;
        }
        y += delta_y;
        x += delta_x;
        x %= m[0].len();
    }

    trees
}

pub fn part1() -> usize {
    let m = load_to_matrix("input/day3.txt")
        .map(|r| r.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    sloaper(&m, 3, 1)
}
pub fn part2() -> usize {
    let m = load_to_matrix("input/day3.txt")
        .map(|r| r.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut ans = 1;
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x, y)| sloaper(&m, x, y))
        .for_each(|v| ans *= v);
    ans
}
