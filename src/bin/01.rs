advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let count = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let first_char = line.chars().next()?;
            let factor = if first_char == 'L' { -1 } else { 1 };
            let number = line[1..].parse::<i32>().ok()?;
            Some(factor * number)
        })
        .scan(50, |position, rotation| {
            *position += rotation;
            *position = ((*position % 100) + 100) % 100;
            Some(*position)
        })
        .filter(|&position| position == 0)
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<i32> {
    let (_, count) = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let first_char = line.chars().next()?;
            let number = line[1..].parse::<i32>().ok()?;
            let full = number / 100;
            let restant = number % 100;
            Some((first_char, full, restant))
        })
        .fold((50, 0), |(current, count), (direction, full, restant)| {
            let next = if direction == 'L' {
                current - restant
            } else {
                current + restant
            };

            let crosses = if current != 0 && (next < 0 || next > 99 || next == 0) {
                1 + full
            } else if current == 0 && (next < 0 || next > 99) {
                full
            } else {
                full
            };

            let new_position = ((next % 100) + 100) % 100;
            (new_position, count + crosses)
        });

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
