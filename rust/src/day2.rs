use std::collections::HashMap;
use std::ops::Index;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
	input.trim().lines().map(|s| s.to_string()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[String]) -> i32 {
	let mut exactly_two = 0;
	let mut exactly_three = 0;

	for id in input {
		let mut char_counts: HashMap<char, u32> = HashMap::new();
		for c in id.chars() {
			char_counts.entry(c).or_insert(0);
			let val = char_counts.index(&c) + 1;
			char_counts.insert(c, val);
		}

		for (_, count) in &char_counts {
			if count == &2 {
				exactly_two += 1;
				break;
			}
		}

		for (_, count) in &char_counts {
			if count == &3 {
				exactly_three += 1;
				break;
			}
		}
	}

	exactly_two * exactly_three
}

#[aoc(day2, part2)]
pub fn part2(input: &[String]) -> String {
	for (i, id_a) in input.iter().enumerate() {
		for j in (i + 1)..input.len() {
			let id_b = input.get(j).unwrap();
			let a_chars: Vec<char> = id_a.chars().collect();
			let b_chars: Vec<char> = id_b.chars().collect();
			let mut diffs = 0;
			let mut same = String::new();
			for n in 0..id_a.len() {
				let char_a = a_chars.get(n).unwrap();
				let char_b = b_chars.get(n).unwrap();
				if char_a == char_b {
					same.push(*char_a);
				} else {
					diffs += 1;
				}
			}

			if diffs == 1 {
				return same;
			}
		}
	}
	"".into()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn part1_sample1() {
		assert_eq!(
			part1(&vec![
				"abcdef".into(),
				"bababc".into(),
				"abbcde".into(),
				"abcccd".into(),
				"aabcdd".into(),
				"abcdee".into(),
				"ababab".into()
			]),
			12
		);
	}

	#[test]
	fn part2_sample1() {
		assert_eq!(
			part2(&vec![
				"abcde".into(),
				"fghij".into(),
				"klmno".into(),
				"pqrst".into(),
				"fguij".into(),
				"axcye".into(),
				"wvxyz".into(),
			]),
			"fgij".to_string()
		);
	}
}
