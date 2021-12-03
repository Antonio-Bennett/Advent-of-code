mod day_01;
mod day_02;
mod day_03;

#[cfg(test)]
mod tests {
    use crate::day_01;
    use crate::day_02;
    use crate::day_03;

    #[test]
    fn day_01() {
        assert_eq!(
            1475,
            day_01::part_a(include_str!("../inputs/day_01/input.txt")).unwrap()
        );

        assert_eq!(
            1516,
            day_01::part_b(include_str!("../inputs/day_01/input.txt")).unwrap()
        );
    }

    #[test]
    fn day_02() {
        assert_eq!(
            1947824,
            day_02::part_a(include_str!("../inputs/day_02/input.txt")).unwrap()
        );

        assert_eq!(
            1813062561,
            day_02::part_b(include_str!("../inputs/day_02/input.txt")).unwrap()
        )
    }

    #[test]
    fn day_03() {
        assert_eq!(
            3985686,
            day_03::part_a(include_str!("../inputs/day_03/input.txt")).unwrap()
        );
    }
}
