fn main() {
    let (max, (x, y)) = max_visibility(PUZZLE_INPUT).unwrap();
    println!("1. With max visibility we see {} asteroids.", max);

    let position = guess_nth(PUZZLE_INPUT, x, y, 200).unwrap();
    println!("2. The 200th destroyed asteroid would be: {:?}", position);
}

#[derive(PartialEq, Debug)]
enum Space {
    Void,
    Asteroid,
}

fn parse_asteroids(input: &str) -> Vec<Vec<Space>> {
    input
        .split('\n')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|piece| {
            piece
                .chars()
                .filter(|c| *c == '#' || *c == '.')
                .map(|c| {
                    if c == '#' {
                        Space::Asteroid
                    } else {
                        Space::Void
                    }
                })
                .collect::<Vec<Space>>()
        })
        .collect()
}

fn max_visibility(input: &str) -> Option<(usize, (usize, usize))> {
    let asteroids = parse_asteroids(input);
    let mut result = None;
    for y in 0..asteroids.len() {
        for x in 0..asteroids[0].len() {
            if asteroids[y][x] == Space::Void {
                continue;
            }
            let detections = sonar_around(&asteroids, x, y);
            let max = if let Some((max, _)) = result { max } else { 0 };
            if detections.len() > max {
                result = Some((detections.len(), (x, y)));
            }
        }
    }
    result
}

const RAYCASTING_RESOLUTION: f64 = 0.04;

const THETA_START: f64 = -180.0 * std::f64::consts::PI * (1.0 / 180.0);
const THETA_END: f64 = THETA_START + 2.0 * std::f64::consts::PI;
const THETA_STEP: f64 = RAYCASTING_RESOLUTION * std::f64::consts::PI * (1.0 / 180.0);

fn sonar_around(asteroids: &[Vec<Space>], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut theta = THETA_END;
    let mut detections = vec![];
    while theta > THETA_START {
        let (tx, ty) = theta.sin_cos();
        if let Some(col) = trace_ray(asteroids, x, y, MVec::new(tx, ty)) {
            if !detections.contains(&col) {
                detections.push(col);
            }
        }
        theta -= THETA_STEP;
    }
    detections
}

const RAY_STEP: f64 = RAYCASTING_RESOLUTION;
const MIN_DISTANCE: f64 = RAYCASTING_RESOLUTION;

fn trace_ray(
    asteroids: &[Vec<Space>],
    origin_x: usize,
    origin_y: usize,
    trajectory: MVec,
) -> Option<(usize, usize)> {
    let mut ray = from_usize_to_f64(origin_x, origin_y);
    while ray.y < asteroids.len() as f64
        && ray.x < asteroids[0].len() as f64
        && ray.x > 0.0
        && ray.y > 0.0
    {
        let (cell_x, cell_y) = from_f64_to_usize(ray);
        ray = ray + trajectory * RAY_STEP;
        if cell_x == origin_x && cell_y == origin_y || asteroids[cell_y][cell_x] != Space::Asteroid
        {
            continue;
        }
        let distance = module(ray - from_usize_to_f64(cell_x, cell_y));
        if distance <= MIN_DISTANCE {
            return Some((cell_x, cell_y));
        }
    }
    None
}

fn guess_nth(input: &str, x: usize, y: usize, n: usize) -> Option<(usize, usize)> {
    let mut asteroids = parse_asteroids(input);
    let mut destroyed = 0;
    loop {
        let detections = sonar_around(&asteroids, x, y);
        if detections.is_empty() {
            break;
        }
        for (x, y) in detections {
            asteroids[y][x] = Space::Void;
            destroyed += 1;
            if destroyed == n {
                return Some((x, y));
            }
        }
    }
    None
}

#[derive(Copy, Clone, Debug)]
struct MVec {
    x: f64,
    y: f64,
}

impl MVec {
    pub fn new(x: f64, y: f64) -> Self {
        MVec { x, y }
    }
}

impl std::ops::Mul<f64> for MVec {
    type Output = MVec;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl std::ops::Add for MVec {
    type Output = MVec;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl std::ops::Sub for MVec {
    type Output = MVec;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

fn from_usize_to_f64(x: usize, y: usize) -> MVec {
    MVec::new(x as f64 + 0.5, y as f64 + 0.5)
}

fn from_f64_to_usize(n: MVec) -> (usize, usize) {
    (n.x as usize, n.y as usize)
}

fn module(n: MVec) -> f64 {
    ((n.x * n.x + n.y * n.y) as f64).sqrt()
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

    fn max_visibility_test(input: &str) -> usize {
        max_visibility(input).unwrap().0
    }

    eq_tests! {
        simple_parsing_1: parse_asteroids("#.\n##") => vec![vec![Space::Asteroid,Space::Void],vec![Space::Asteroid,Space::Asteroid]];
        simple_parsing_2: parse_asteroids("..\n##") => vec![vec![Space::Void,Space::Void],vec![Space::Asteroid,Space::Asteroid]];
        example_1: max_visibility_test(".#..#
        .....
        #####
        ....#
        ...##") => 8;
        example_2: max_visibility_test("......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####") => 33;
        example_3: max_visibility_test("#.#...#.#.
        .###....#.
        .#....#...
        ##.#.#.#.#
        ....#.#.#.
        .##..###.#
        ..#...##..
        ..##....##
        ......#...
        .####.###.") => 35;
        example_4: max_visibility_test(".#..#..###
        ####.###.#
        ....###.#.
        ..###.##.#
        ##.##.#.#.
        ....###..#
        ..#.#..#.#
        #..#.#.###
        .##...##.#
        .....#.#..") => 41;
        example_5: max_visibility_test(".#..##.###...#######
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
        part_1: max_visibility_test(PUZZLE_INPUT) => 230;
        sonar_around_clockwise: sonar_around(&parse_asteroids("###
        ###
        ###"), 1, 1).into_iter().collect::<Vec<_>>() => vec![(1, 0), (2, 0), (2, 1), (2, 2), (1, 2), (0, 2), (0, 1), (0, 0)];
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
