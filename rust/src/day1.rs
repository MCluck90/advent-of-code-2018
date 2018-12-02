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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sample1() {
    assert_eq!(part1(&vec![1, -2, 3, 1]), 3);
  }

  #[test]
  fn sample2() {
    assert_eq!(part1(&vec![1, 1, 1]), 3);
  }

  #[test]
  fn sample3() {
    assert_eq!(part1(&vec![1, 1, -2]), 0);
  }

  #[test]
  fn sample4() {
    assert_eq!(part1(&vec![-1, -2, -3]), -6);
  }
}
