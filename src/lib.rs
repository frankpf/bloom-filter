extern crate bit_vec;
extern crate seahash;
use bit_vec::BitVec;

pub struct BloomFilter {
	buf: BitVec,
	len: usize,
	salts: [u64; 2],
}

impl BloomFilter {
	pub fn new(len: usize) -> BloomFilter {
		BloomFilter {
			buf: BitVec::from_elem(len, false),
			len,
			salts: [0, 1],
		}
	}

	pub fn from_vec(vec: &[&str]) -> BloomFilter {
		let mut bf = BloomFilter::new(1000);
		for it in vec.iter() {
			bf.insert(it);
		}

		bf
	}

	fn get_salted_strings(&self, item: &str) -> (String, String) {
		let s1 = format!("{}{}", item, self.salts[0]);
		let s2 = format!("{}{}", item, self.salts[1]);

		(s1, s2)
	}

	fn query_bytes(&self, bytes: &[u8]) -> Option<bool> {
		let hash = seahash::hash(bytes);
		let bit = (hash as usize) % self.len;

		self.buf.get(bit)
	}

	fn insert_bytes(&mut self, bytes: &[u8]) {
		let hash = seahash::hash(bytes);
		let bit = (hash as usize) % self.len;

		self.buf.set(bit, true);
	}

	pub fn query(&self, item: &str) -> bool {
		let (s1, s2) = self.get_salted_strings(item);
		let b1 = self.query_bytes(s1.as_bytes());
		let b2 = self.query_bytes(s2.as_bytes());
		match (b1, b2) {
			(Some(true), Some(true)) => true,
			(_, _)                   => false
		}
	}

	pub fn insert(&mut self, item: &str) {
		let (s1, s2) = self.get_salted_strings(item);

		self.insert_bytes(s1.as_bytes());
		self.insert_bytes(s2.as_bytes());
	}

	pub fn clear(&mut self) {
		self.buf.clear();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bloom_filter_test() {
		let input = vec!["Lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipisicing", "elit", "sed",
		"do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore", "magna", "aliqua", "a",
		"b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t",
		"u", "v", "w", "x", "y", "z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10" ];

		let bf = BloomFilter::from_vec(&input);

		for word in input.iter() {
			assert!(bf.query(&format!("{}{}",word,0)));
		}

		assert!(!bf.query("missing"));
		assert!(!bf.query("fizzbuzz"));
	}
}

