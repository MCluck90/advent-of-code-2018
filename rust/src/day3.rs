#[derive(Debug, PartialEq)]
pub struct Rect {
	pub id: u32,
	pub x: u32,
	pub y: u32,
	pub w: u32,
	pub h: u32,
}

impl Rect {
	pub fn overlap(&self, other: &Rect) -> Option<Rect> {
		use std::cmp::{max, min};
		if self.x < other.x + other.w
			&& self.x + self.w > other.x
			&& self.y < other.y + other.w
			&& self.y + self.h > other.y
		{
			let self_right = self.x + self.w;
			let other_right = other.x + other.w;
			let self_bottom = self.y + self.h;
			let other_bottom = other.y + other.h;
			let x = max(self.x, other.x);
			let y = max(self.y, other.y);
			let w = min(self_right, other_right) - x;
			let h = min(self_bottom, other_bottom) - y;
			Some(Rect { id: 0, x, y, w, h })
		} else {
			None
		}
	}
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Rect> {
	let mut claims = Vec::new();

	for line in input.lines().map(|l| l.trim()) {
		let mut pieces = line.split(' ');
		// Pull out the unparsed sections
		let id = pieces.next().unwrap();
		let _ = pieces.next().unwrap(); // Ignore the @
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
	fn complete_overlap() {
		let first = Rect {
			id: 0,
			x: 0,
			y: 0,
			w: 10,
			h: 10,
		};
		let second = Rect {
			id: 0,
			x: 0,
			y: 0,
			w: 10,
			h: 10,
		};
		let overlap = first.overlap(&second).unwrap();
		assert_eq!(overlap.x, 0);
		assert_eq!(overlap.y, 0);
		assert_eq!(overlap.w, 10);
		assert_eq!(overlap.h, 10);
	}

	#[test]
	fn partial_overlap() {
		let first = Rect {
			id: 0,
			x: 0,
			y: 0,
			w: 10,
			h: 10,
		};
		let second = Rect {
			id: 0,
			x: 5,
			y: 5,
			w: 10,
			h: 10,
		};
		let overlap = first.overlap(&second).unwrap();
		assert_eq!(overlap.x, 5);
		assert_eq!(overlap.y, 5);
		assert_eq!(overlap.w, 5);
		assert_eq!(overlap.h, 5);
	}

	#[test]
	fn complete_internal_overlap() {
		let first = Rect {
			id: 0,
			x: 0,
			y: 0,
			w: 10,
			h: 10,
		};
		let second = Rect {
			id: 0,
			x: 2,
			y: 2,
			w: 3,
			h: 3,
		};
		let overlap = first.overlap(&second).unwrap();
		assert_eq!(overlap.x, 2);
		assert_eq!(overlap.y, 2);
		assert_eq!(overlap.w, 3);
		assert_eq!(overlap.h, 3);
	}

	#[test]
	fn no_overlap() {
		let first = Rect {
			id: 0,
			x: 0,
			y: 0,
			w: 10,
			h: 10,
		};
		let second = Rect {
			id: 0,
			x: 20,
			y: 20,
			w: 10,
			h: 10,
		};
		let overlap = first.overlap(&second);
		assert_eq!(overlap, None);
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
