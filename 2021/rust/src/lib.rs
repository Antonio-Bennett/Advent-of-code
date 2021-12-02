mod day_01;

#[cfg(test)]
mod tests {
    use super::day_01::part_a;

    #[test]
    fn day_01_a() {
        assert_eq!(
            1475,
            part_a(include_str!("../inputs/day_01/input.txt")).unwrap()
        );
    }
}
