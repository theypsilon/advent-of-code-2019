fn main() {
    let total_fuel = PUZZLE_INPUT
        .iter()
        .map(|mass| fuel_from_mass(*mass))
        .fold(0, |acc, fuel| acc + fuel);
    println!("1. total_fuel: {}", total_fuel);

    let total_fuel = PUZZLE_INPUT
        .iter()
        .map(|mass| fuel_from_module(*mass))
        .fold(0, |acc, fuel| acc + fuel);
    println!("2. total_fuel: {}", total_fuel);

}

fn fuel_from_mass(mass: i64) -> i64 {
    let result = (mass / 3) - 2;
    if result <= 0 { 0 } else { result }
}

fn fuel_from_module(mut mass: i64) -> i64 {
    let mut result = 0;
    while mass > 0 {
        let fuel = fuel_from_mass(mass);
        result += fuel;
        mass = fuel;
    }
    result
}

#[cfg(test)]
mod test {
    use super::fuel_from_mass;
    #[test]
    fn test_fuel_from_mass_1_star() {
        assert_eq!(fuel_from_mass(12), 2);
        assert_eq!(fuel_from_mass(14), 2);
        assert_eq!(fuel_from_mass(1969), 654);
        assert_eq!(fuel_from_mass(100756), 33583);
    }

    #[test]
    fn test_fuel_from_mass_2_star() {
        assert_eq!(fuel_from_mass(2), 0);
    }

    use super::fuel_from_module;
    #[test]
    fn test_fuel_from_module() {
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
