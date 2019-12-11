use std::collections::HashSet;

fn main() {
    let max = max_visibility(PUZZLE_INPUT);
    println!("1. max: {}", max);
}

fn parse_asteroids(input: &str) -> Vec<Vec<i64>> {
    input
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|piece| {
            piece
                .chars()
                .filter(|c| *c == '#' || *c == '.')
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn max_visibility(input: &str) -> i64 {
    let asteroids = parse_asteroids(input);
    let mut max = -1;
    for y in 0..asteroids.len() {
        for x in 0..asteroids[0].len() {
            if asteroids[y][x] == 0 {
                continue;
            }
            let visibility = get_visibility(&asteroids, x as i64, y as i64);
            if visibility > max {
                max = visibility;
            }
        }
    }
    max
}

fn get_visibility(asteroids: &[Vec<i64>], i: i64, j: i64) -> i64 {
    let mut set: HashSet<(i64, i64)> = HashSet::new();
    for y in 0..asteroids.len() {
        for x in 0..asteroids[0].len() {
            let dx = i - x as i64;
            let dy = j - y as i64;
            if (dx == 0 && dy == 0) || asteroids[y][x] == 0 {
                continue;
            }
            set.insert(to_i64(normalize((dx as f64, dy as f64))));
        }
    }
    set.len() as i64
}

const INT_FACTOR: f64 = 10_000.0;

fn to_i64(n: (f64, f64)) -> (i64, i64) {
    ((n.0 * INT_FACTOR) as i64, (n.1 * INT_FACTOR) as i64)
}

#[allow(dead_code)]
fn to_f64(n: (i64, i64)) -> (f64, f64) {
    ((n.0 as f64 / INT_FACTOR), (n.1 as f64 / INT_FACTOR))
}

fn module(n: (f64, f64)) -> f64 {
    ((n.0 * n.0 + n.1 * n.1) as f64).sqrt()
}

fn normalize(n: (f64, f64)) -> (f64, f64) {
    let m = module(n);
    let x = n.0 / m;
    let y = n.1 / m;
    (x, y)
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! eq_tests {
        ( $( $name:ident: $input:expr => $expected:expr;)* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($input, $expected);
                }
            )*
        };
    }

    eq_tests! {
        simple_parsing_1: parse_asteroids("#.\n##") => vec![vec![1,0],vec![1,1]];
        simple_parsing_2: parse_asteroids("..\n##") => vec![vec![0,0],vec![1,1]];
        example_1: max_visibility(".#..#
        .....
        #####
        ....#
        ...##") => 8;
        example_2: max_visibility("......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####") => 33;
        example_3: max_visibility("#.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.") => 35;
        example_4: max_visibility(".#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..") => 41;
        example_5: max_visibility(".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##") => 210;
    }
}

const PUZZLE_INPUT: &str = ".###..#......###..#...#
#.#..#.##..###..#...#.#
#.#.#.##.#..##.#.###.##
.#..#...####.#.##..##..
#.###.#.####.##.#######
..#######..##..##.#.###
.##.#...##.##.####..###
....####.####.#########
#.########.#...##.####.
.#.#..#.#.#.#.##.###.##
#..#.#..##...#..#.####.
.###.#.#...###....###..
###..#.###..###.#.###.#
...###.##.#.##.#...#..#
#......#.#.##..#...#.#.
###.##.#..##...#..#.#.#
###..###..##.##..##.###
###.###.####....######.
.###.#####.#.#.#.#####.
##.#.###.###.##.##..##.
##.#..#..#..#.####.#.#.
.#.#.#.##.##########..#
#####.##......#.#.####.";
