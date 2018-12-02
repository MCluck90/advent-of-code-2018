use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
  input
    .trim()
    .lines()
    .map(|l| l.parse::<i32>().unwrap())
    .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
  input.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
  let mut seen = HashSet::new();
  let mut frequency = 0;
  seen.insert(0);
  loop {
    for val in input {
      frequency += val;
      if seen.contains(&frequency) {
        return frequency;
      }
      seen.insert(frequency);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_sample1() {
    assert_eq!(part1(&vec![1, -2, 3, 1]), 3);
  }

  #[test]
  fn part1_sample2() {
    assert_eq!(part1(&vec![1, 1, 1]), 3);
  }

  #[test]
  fn part1_sample3() {
    assert_eq!(part1(&vec![1, 1, -2]), 0);
  }

  #[test]
  fn part1_sample4() {
    assert_eq!(part1(&vec![-1, -2, -3]), -6);
  }

  #[test]
  fn part2_sample1() {
    assert_eq!(part2(&vec![1, -1]), 0);
  }

  #[test]
  fn part2_sample2() {
    assert_eq!(part2(&vec![3, 3, 4, -2, -4]), 10);
  }

  #[test]
  fn part2_sample3() {
    assert_eq!(part2(&vec![-6, 3, 8, 5, -6]), 5);
  }

  #[test]
  fn part2_sample4() {
    assert_eq!(part2(&vec![7, 7, -2, -7, -4]), 14);
  }
}
