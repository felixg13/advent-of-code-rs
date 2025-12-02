advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .split(',')
        .filter_map(|str| str.split_once('-'))
        .flat_map(|(min, max)| {
            let min = min.trim().parse::<u64>().unwrap();
            let max = max.trim().parse::<u64>().unwrap();
            min..=max
        })
        .filter(|&i| {
            let num_digits = i.to_string().len();
            let n = num_digits / 2;
            let divisor = 10_u64.pow(n as u32);
            let left = i / divisor;
            let right = i % divisor;
            left == right
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum = input
        .split(',')
        .filter_map(|str| str.split_once('-'))
        .flat_map(|(min, max)| {
            let min = min.trim().parse::<u64>().unwrap();
            let max = max.trim().parse::<u64>().unwrap();
            min..=max
        })
        .flat_map(|i| {
            let s = i.to_string();
            let len = s.len();
            (1..len).filter_map(move |chunk_size| {
                if len % chunk_size != 0 {
                    return None;
                }
                let chunks: Vec<u64> = s
                    .as_bytes()
                    .chunks(chunk_size)
                    .filter_map(|chunk| std::str::from_utf8(chunk).ok()?.parse::<u64>().ok())
                    .collect();
                Some(chunks)
            })
        })
        .filter(|chunks| chunks.len() > 1 && chunks.iter().all(|&x| x == chunks[0]))
        .map(|chunks| {
            chunks
                .iter()
                .map(|x| x.to_string())
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
