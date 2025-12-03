use itertools::Itertools;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .lines()
        .filter_map(|line| {
            line.chars()
                .combinations(2)
                .filter_map(|combo| combo.iter().collect::<String>().parse::<u64>().ok())
                .max()
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .filter_map(|line| {
            let chars: Vec<char> = line.chars().collect();
            let n = chars.len();

            if 12 > n {
                return None;
            }

            let mut result = String::new();
            let mut start = 0;

            for picks_left in (1..=12).rev() {
                let end = n - picks_left + 1;

                let max_char = chars[start..end].iter().max()?;
                let max_pos = start + chars[start..end].iter().position(|c| c == max_char)?;

                result.push(*max_char);
                start = max_pos + 1;
            }

            result.parse::<u64>().ok()
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
