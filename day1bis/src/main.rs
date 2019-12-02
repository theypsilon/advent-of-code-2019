fn main() {
    let total_fuel = PUZZLE_INPUT
        .iter()
        .cloned()
        .map(fuel_from_mass)
        .fold(0, std::ops::Add::add);
    println!("1. total_fuel: {}", total_fuel);

    let total_fuel = PUZZLE_INPUT
        .iter()
        .cloned()
        .map(fuel_from_module)
        .fold(0, std::ops::Add::add);
    println!("2. total_fuel: {}", total_fuel);
}

fn fuel_from_mass(mass: i64) -> i64 {
    mass / 3 - 2
}

fn fuel_from_module(mass: i64) -> i64 {
    let fuel = fuel_from_mass(mass);
    if fuel < 0 { 0 } else { fuel + fuel_from_module(fuel) }
}

#[cfg(test)]
mod test {

    use super::fuel_from_mass;

    #[test]
    fn test_fuel_from_mass() {
        assert_eq!(fuel_from_mass(12), 2);
        assert_eq!(fuel_from_mass(14), 2);
        assert_eq!(fuel_from_mass(1969), 654);
        assert_eq!(fuel_from_mass(100756), 33583);
    }

    use super::fuel_from_module;
    #[test]
    fn test_fuel_from_module() {
        assert_eq!(fuel_from_module(14), 2);
        assert_eq!(fuel_from_module(1969), 966);
        assert_eq!(fuel_from_module(100756), 50346);
    }
}

const PUZZLE_INPUT: [i64; 100] = [
    66452, 116352, 149063, 89740, 127871, 67079, 110072, 69113, 81350, 78546, 60987, 135761,
    124758, 88974, 62785, 95781, 142073, 112941, 50611, 60254, 119624, 113248, 79006, 64084,
    112574, 93665, 70195, 123125, 131451, 129048, 134267, 60878, 131790, 129317, 80881, 63994,
    116531, 61733, 68840, 94325, 55880, 95804, 85840, 81390, 105875, 52840, 129801, 93510, 60717,
    129838, 84428, 78677, 108652, 68968, 74477, 131263, 113174, 79762, 125274, 71145, 104933,
    113211, 81420, 74843, 121886, 83881, 101605, 119888, 60893, 137917, 100729, 54363, 120755,
    148169, 63014, 82424, 100093, 60746, 76765, 127239, 121852, 124982, 114509, 147435, 55606,
    67360, 93258, 108443, 98212, 52320, 135855, 51583, 109452, 143535, 123262, 130966, 121649,
    99241, 82066, 60047,
];
