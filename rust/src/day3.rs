#[derive(Debug, PartialEq)]
pub struct Rect {
	id: u32,
	x: u32,
	y: u32,
	w: u32,
	h: u32,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rect> {
	let mut claims = Vec::new();

	for line in input.lines().map(|l| l.trim()) {
		let mut pieces = line.split(' ');
		// Pull out the unparsed sections
		let id = pieces.next().unwrap();
		let _ = pieces.next().unwrap();
		let mut coords = pieces.next().unwrap().split(',');
		let mut size = pieces.next().unwrap().split('x');

		// Parse out the individual pieces
		let id: u32 = id[1..].parse().unwrap();
		let x = coords.next().unwrap();
		let x: u32 = x.parse().unwrap();
		let y = coords.next().unwrap();
		let y: u32 = y[0..(y.len() - 1)].parse().unwrap();
		let w = size.next().unwrap();
		let w: u32 = w.parse().unwrap();
		let h = size.next().unwrap();
		let h: u32 = h.parse().unwrap();

		claims.push(Rect { id, x, y, w, h });
	}

	claims
}

#[aoc(day3, part1)]
pub fn part1(_claims: &[Rect]) -> u32 {
	0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_input_parser() {
		let input = "#1 @ 2,4: 8x16";
		let output = input_generator(input);
		let expected = Rect {
			id: 1,
			x: 2,
			y: 4,
			w: 8,
			h: 16,
		};
		assert_eq!(output[0], expected);
	}

	#[test]
	fn part1_sample1() {
		let input = "#1 @ 1,3: 4x4
		#2 @ 3,1: 4x4
		#3 @ 5,5: 2x2";
		let input = input_generator(input);
		let output = part1(&input);
		assert_eq!(output, 4);
	}
}
