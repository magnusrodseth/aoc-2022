fn marker<const CHUNK_SIZE: usize>(input: &str) -> usize {
    input
        .as_bytes()
        .windows(CHUNK_SIZE)
        .position(|chunk| {
            let mut chunk_range = 0..(CHUNK_SIZE - 1);

            !chunk_range.any(|i| {
                let mut rest_range = (i + 1)..CHUNK_SIZE;
                rest_range.any(|j| chunk[i] == chunk[j])
            })
        })
        .expect("No solution found")
        + CHUNK_SIZE
}

fn part1(input: &str) -> usize {
    marker::<4>(input)
}

fn part2(input: &str) -> usize {
    marker::<14>(input)
}

pub fn solve() {
    let input = include_str!("../../input/day06.txt");

    println!("Day 1 Part 1: {:?}", part1(input));
    println!("Day 1 Part 2: {:?}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    type Actual<'a> = &'a str;
    type Expected = usize;
    type TestCase<'a> = (Actual<'a>, Expected);

    #[test]
    fn day06_part1() {
        let input1: TestCase = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7);
        let input2: TestCase = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5);
        let input3: TestCase = ("nppdvjthqldpwncqszvftbrmjlhg", 6);
        let input4: TestCase = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10);
        let input5: TestCase = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11);

        let (input, expected) = input1;
        assert_eq!(part1(input), expected);
        let (input, expected) = input2;
        assert_eq!(part1(input), expected);
        let (input, expected) = input3;
        assert_eq!(part1(input), expected);
        let (input, expected) = input4;
        assert_eq!(part1(input), expected);
        let (input, expected) = input5;
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn day06_part2() {
        let input1: TestCase = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19);
        let input2: TestCase = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23);
        let input3: TestCase = ("nppdvjthqldpwncqszvftbrmjlhg", 23);
        let input4: TestCase = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29);
        let input5: TestCase = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26);

        let (input, expected) = input1;
        assert_eq!(part2(input), expected);
        let (input, expected) = input2;
        assert_eq!(part2(input), expected);
        let (input, expected) = input3;
        assert_eq!(part2(input), expected);
        let (input, expected) = input4;
        assert_eq!(part2(input), expected);
        let (input, expected) = input5;
        assert_eq!(part2(input), expected);
    }
}
