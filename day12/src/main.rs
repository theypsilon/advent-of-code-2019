use std::collections::HashSet;
use std::ops::Rem;

fn main() {
    let energy = calculate_energy(run_simulation(PUZZLE_INPUT, 1000));
    println!("1. energy: {}", energy);

    let ticks = run_until_first_state(PUZZLE_INPUT);
    println!("2. ticks: {}", ticks);
}

fn run_until_first_state(input: &str) -> usize {
    let moons: Vec<_> = parse_vectors(input)
        .into_iter()
        .map(Moon::new_not_moving)
        .collect();
    let couples = unique_couples(&(0..moons.len()).collect::<Vec<usize>>());
    let soa_moons = Moons::from_aos(moons);
    let initial_state = soa_moons.clone();
    let Moons {
        mut pos_x,
        mut vel_x,
        mut pos_y,
        mut vel_y,
        mut pos_z,
        mut vel_z,
    } = soa_moons;
    let ticks_x = run_until_match(
        &couples,
        &mut pos_x,
        &mut vel_x,
        (&initial_state.pos_x, &initial_state.vel_x),
    );
    let ticks_y = run_until_match(
        &couples,
        &mut pos_y,
        &mut vel_y,
        (&initial_state.pos_y, &initial_state.vel_y),
    );
    let ticks_z = run_until_match(
        &couples,
        &mut pos_z,
        &mut vel_z,
        (&initial_state.pos_z, &initial_state.vel_z),
    );
    lcm(lcm(ticks_x, ticks_y), ticks_z)
}

fn run_until_match(
    couples: &[(usize, usize)],
    pos: &mut [V3DComponent],
    vel: &mut [V3DComponent],
    other: (&[V3DComponent], &[V3DComponent]),
) -> usize {
    let mut ticks = 0;
    loop {
        iterate_couples_split(couples, pos, vel);
        ticks += 1;
        if equals(&pos, &vel, &other.0, &other.1) {
            break;
        }
    }
    ticks
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

pub trait NonNegativeInteger: Copy + PartialOrd + Rem<Output = Self> + From<u8> {}

impl NonNegativeInteger for u8 {}
impl NonNegativeInteger for u16 {}
impl NonNegativeInteger for u32 {}
impl NonNegativeInteger for u64 {}
impl NonNegativeInteger for u128 {}
impl NonNegativeInteger for usize {}

pub fn gcd<T: NonNegativeInteger>(a: T, b: T) -> T {
    let (mut a, mut b) = if a > b { (a, b) } else { (b, a) };
    while b != 0.into() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn unique_couples(slice: &[usize]) -> Vec<(usize, usize)> {
    let mut set = HashSet::new();
    for i in slice {
        for j in slice {
            if i == j {
                continue;
            }
            if i < j {
                set.insert((*i, *j));
            } else {
                set.insert((*j, *i));
            }
        }
    }
    let mut vec: Vec<_> = set.into_iter().collect();
    vec.sort();
    vec
}

fn calculate_energy(moons: Vec<Moon>) -> V3DComponent {
    moons
        .into_iter()
        .map(|m| {
            (m.pos.x.abs() + m.pos.y.abs() + m.pos.z.abs())
                * (m.vel.x.abs() + m.vel.y.abs() + m.vel.z.abs())
        })
        .fold(0 as V3DComponent, std::ops::Add::add)
}

fn run_simulation(input: &str, total_ticks: usize) -> Vec<Moon> {
    let mut moons_1: Vec<_> = parse_vectors(input)
        .into_iter()
        .map(Moon::new_not_moving)
        .collect();

    let couples = unique_couples(&(0..moons_1.len()).collect::<Vec<usize>>());

    for _ in 0..total_ticks {
        iterate_couples(&couples, &mut moons_1);
    }
    moons_1
}

fn iterate_couples(couples: &[(usize, usize)], moons: &mut [Moon]) {
    for (i, j) in couples {
        if moons[*j].pos.x > moons[*i].pos.x {
            moons[*i].vel.x += 1;
            moons[*j].vel.x -= 1;
        } else if moons[*j].pos.x < moons[*i].pos.x {
            moons[*i].vel.x -= 1;
            moons[*j].vel.x += 1;
        }
        if moons[*j].pos.y > moons[*i].pos.y {
            moons[*i].vel.y += 1;
            moons[*j].vel.y -= 1;
        } else if moons[*j].pos.y < moons[*i].pos.y {
            moons[*i].vel.y -= 1;
            moons[*j].vel.y += 1;
        }
        if moons[*j].pos.z > moons[*i].pos.z {
            moons[*i].vel.z += 1;
            moons[*j].vel.z -= 1;
        } else if moons[*j].pos.z < moons[*i].pos.z {
            moons[*i].vel.z -= 1;
            moons[*j].vel.z += 1;
        }
    }
    for moon in moons.iter_mut() {
        moon.pos.x += moon.vel.x;
        moon.pos.y += moon.vel.y;
        moon.pos.z += moon.vel.z;
    }
}

fn iterate_couples_split(
    couples: &[(usize, usize)],
    pos: &mut [V3DComponent],
    vel: &mut [V3DComponent],
) {
    for (i, j) in couples {
        if pos[*j] > pos[*i] {
            vel[*i] += 1;
            vel[*j] -= 1;
        } else if pos[*j] < pos[*i] {
            vel[*i] -= 1;
            vel[*j] += 1;
        }
    }
    for i in 0..pos.len() {
        pos[i] += vel[i];
    }
}

#[derive(Clone)]
struct Moons {
    pos_x: Vec<V3DComponent>,
    pos_y: Vec<V3DComponent>,
    pos_z: Vec<V3DComponent>,
    vel_x: Vec<V3DComponent>,
    vel_y: Vec<V3DComponent>,
    vel_z: Vec<V3DComponent>,
}

impl Moons {
    pub fn from_aos(moons: Vec<Moon>) -> Self {
        let pos_x = moons.iter().map(|m| m.pos.x).collect();
        let pos_y = moons.iter().map(|m| m.pos.y).collect();
        let pos_z = moons.iter().map(|m| m.pos.z).collect();
        let vel_x = moons.iter().map(|m| m.vel.x).collect();
        let vel_y = moons.iter().map(|m| m.vel.y).collect();
        let vel_z = moons.iter().map(|m| m.vel.z).collect();
        Moons {
            pos_x,
            pos_y,
            pos_z,
            vel_x,
            vel_y,
            vel_z,
        }
    }
}

pub fn equals(
    pos: &[V3DComponent],
    vel: &[V3DComponent],
    other_pos: &[V3DComponent],
    other_vel: &[V3DComponent],
) -> bool {
    pos == other_pos && vel == other_vel
}

#[derive(Debug, PartialEq, Clone)]
struct Moon {
    pos: V3D,
    vel: V3D,
}

impl Moon {
    pub fn new(pos: V3D, vel: V3D) -> Self {
        Moon { pos, vel }
    }

    pub fn new_not_moving(pos: V3D) -> Self {
        Moon {
            pos,
            vel: V3D::new(0, 0, 0),
        }
    }
}

fn parse_vectors(input: &str) -> Vec<V3D> {
    input
        .split('>')
        .filter_map(|piece| {
            let line = piece
                .chars()
                .filter(|c| match c {
                    ' ' | '\n' | '<' | ',' | '>' => false,
                    _ => true,
                })
                .collect::<Vec<_>>();
            let mut x = None;
            let mut y = None;
            let mut z = None;
            let mut i = 0;
            while i < line.len() {
                match line[i] {
                    'x' => {
                        let (new_x, new_i) = parse_number(&line, i + 2);
                        x = new_x;
                        i = new_i;
                    }
                    'y' => {
                        let (new_y, new_i) = parse_number(&line, i + 2);
                        y = new_y;
                        i = new_i;
                    }
                    'z' => {
                        let (new_z, _) = parse_number(&line, i + 2);
                        z = new_z;
                        break;
                    }
                    capture => panic!("Wrong vector component: {}", capture),
                }
            }
            if let Some(x) = x {
                if let Some(y) = y {
                    if let Some(z) = z {
                        return Some(V3D::new(x, y, z));
                    }
                }
            }
            None
        })
        .collect()
}

fn parse_number(line: &[char], mut i: usize) -> (Option<V3DComponent>, usize) {
    let minus;
    match line[i] {
        '-' => {
            minus = true;
            i += 1;
        }
        _ => minus = false,
    }
    let mut number: Option<V3DComponent> = None;
    while i < line.len() {
        match line[i] {
            n @ '0'..='9' => {
                if number.is_none() {
                    number = Some(0);
                }
                number = Some(number.unwrap() * 10 + n.to_digit(10).unwrap() as V3DComponent);
                i += 1;
            }
            _ => {
                break;
            }
        }
    }
    if let Some(number) = number {
        (
            Some(if minus {
                -(number as V3DComponent)
            } else {
                number as V3DComponent
            }),
            i,
        )
    } else {
        (None, i)
    }
}

type V3DComponent = i32;

#[derive(Clone, Debug, PartialEq)]
struct V3D {
    x: V3DComponent,
    y: V3DComponent,
    z: V3DComponent,
}

impl V3D {
    pub fn new(x: V3DComponent, y: V3DComponent, z: V3DComponent) -> Self {
        V3D { x, y, z }
    }
}

impl std::ops::Mul<V3DComponent> for V3D {
    type Output = V3D;
    fn mul(mut self, rhs: V3DComponent) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl std::ops::Add for V3D {
    type Output = V3D;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl std::ops::Sub for V3D {
    type Output = V3D;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
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

    fn moon(
        x1: V3DComponent,
        y1: V3DComponent,
        z1: V3DComponent,
        x2: V3DComponent,
        y2: V3DComponent,
        z2: V3DComponent,
    ) -> Moon {
        Moon::new(V3D::new(x1, y1, z1), V3D::new(x2, y2, z2))
    }

    eq_tests! {
        parse_vectors_1: parse_vectors(FIXTURE_EXAMPLE_1) => vec![V3D::new(-1, 0, 2), V3D::new(2, -10, -7), V3D::new(4, -8, 8), V3D::new(3, 5, -1)];
        example_iteration_0: run_simulation(FIXTURE_EXAMPLE_1, 0) => vec![moon(-1, 0,2,0,0,0), moon(2,-10,-7,0,0,0), moon(4,-8,8,0,0,0), moon(3,5,-1,0,0,0)];
        example_iteration_1: run_simulation(FIXTURE_EXAMPLE_1, 1) => vec![moon(2, -1,1,3,-1,-1), moon(3,-7,-4,1,3,3), moon(1,-7,5,-3,1,-3), moon(2,2,0,-1,-3,1)];
        example_energy_10: calculate_energy(run_simulation(FIXTURE_EXAMPLE_1, 10)) => 179;
        unique_couples_1: unique_couples(&vec![0, 1, 2]) => vec![(0, 1), (0, 2), (1, 2)];
    }
}

const FIXTURE_EXAMPLE_1: &str = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>
";

const PUZZLE_INPUT: &str = "
<x=3, y=15, z=8>
<x=5, y=-1, z=-2>
<x=-10, y=8, z=2>
<x=8, y=4, z=-5>
";
