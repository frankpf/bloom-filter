extern crate bit_vec;
extern crate seahash;
use bit_vec::BitVec;
#[allow(dead_code)]
pub struct BloomFilter {
	buf: BitVec,
	len: usize,
	salts: [u64; 2],
}

impl BloomFilter {
	pub fn new(len: usize) -> BloomFilter {
		BloomFilter {
			buf: BitVec::from_elem(len, false),
			len: len,
			salts: [0, 1],
		}
	}

	pub fn query(&self, item: &str) -> bool {
		let mut s1 = "".to_owned();
		s1.push_str(item);
		s1.push_str(&self.salts[0].to_string());
		println!("Querying bytes for {}", s1);
		let b1 = self.query_bytes(s1.as_bytes());

		let mut s2 = "".to_owned();
		s2.push_str(item);
		s2.push_str(&self.salts[1].to_string());
		println!("Querying bytes for {}", s2);
		let b2 = self.query_bytes(s2.as_bytes());

		match b1 {
			Some(true) => true,
			_ => match b2 {
				Some(true) => true,
				_ => false
			}
		}
	}

	pub fn query_bytes(&self, bytes: &[u8]) -> Option<bool> {
		let hash = seahash::hash(bytes);
		let bit = (hash as usize) % self.len;
		println!("Hash: {}, Hash(usize): {}, Bit: {}", hash, hash as usize, bit);
		self.buf.get(bit)
	}

	pub fn insert_bytes(&mut self, bytes: &[u8]) {
		let hash = seahash::hash(bytes);
		let bit = (hash as usize) % self.len;
		println!("Hash: {}, Hash(usize): {}, bit: {}", hash, hash as usize, bit);
		self.buf.set(bit, true);
	}

	pub fn insert(&mut self, item: &str) {
		let mut s1 = "".to_owned();
		s1.push_str(item);
		s1.push_str(&self.salts[0].to_string());
		println!("Setting bytes for {}", s1);
		self.insert_bytes(s1.as_bytes());

		let mut s2 = "".to_owned();
		s2.push_str(item);
		s2.push_str(&self.salts[1].to_string());
		println!("Setting bytes for {}", s2);
		self.insert_bytes(s2.as_bytes());
	}

	pub fn clear(&mut self) {
		self.buf.clear();
	}
}

#[test]
fn bloom_filter_test() {
	let input = ["Lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipisicing", "elit", "sed",
	"do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore", "magna", "aliqua"];

	let mut bf = BloomFilter::new(100);
	for word in input.iter() { 
		bf.insert(word);
	}

	for word in input.iter() {
		assert!(bf.query(word));
	}

	assert!(!bf.query("missing"));
	assert!(!bf.query("fizzbuzz"));
}
