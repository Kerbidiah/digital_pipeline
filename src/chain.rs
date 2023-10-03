use bytes::{Bytes, Buf};

#[derive(Debug)]
pub struct Chain {
	arr: Vec<Bytes>,
	// which Bytes thingy we are in
	index: usize,
	bytes_remaining: usize,
}

impl Chain {
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			arr: Vec::with_capacity(capacity),
			index: 0,
			bytes_remaining: 0,
		}
	}

	pub fn new() -> Self {
		Self::with_capacity(0)
	}

	pub fn push(&mut self, value: Bytes) {
		self.bytes_remaining += value.remaining();
		self.arr.push(value);
	}

	pub fn append(&mut self, mut other: Self) {
		self.bytes_remaining += other.bytes_remaining;
		self.arr.append(&mut other.arr);
	}

	pub fn append_from_vec(&mut self, other: Vec<Bytes>) {
		self.append(other.into())
	}
	
	#[inline]
	fn current(&self) -> &Bytes {
		&self.arr[self.index]
	}

	#[inline]
	fn advance_current(&mut self, cnt: usize) {
		self.bytes_remaining -= cnt;
		self.arr[self.index].advance(cnt);
	}

	#[inline]
	fn current_remaining(&self) -> usize {
		self.current().remaining()
	}

	/// advances to the next Bytes, returning how many bytes were skiped (`self.current_remaining()`)
	#[inline]
	fn advance_to_next(&mut self) -> usize {
		let cnt = self.current_remaining();
		self.advance_current(cnt);
		self.index += 1;

		cnt
	}
}

impl From<Vec<Bytes>> for Chain {
	fn from(value: Vec<Bytes>) -> Self {
		let bytes_remaining = value.iter()
			.map(|each| each.remaining())
			.sum();

		Self {
			arr: value,
			index: 0,
			bytes_remaining
		}
	}
}

impl Buf for Chain {
	#[inline]
	fn remaining(&self) -> usize {
		self.bytes_remaining
	}

	fn chunk(&self) -> &[u8] {
		self.current().chunk()
	}

	fn advance(&mut self, mut cnt: usize) {
		assert!(
			cnt <= self.remaining(),
			"cannot advance past `end`: {:?} <= {:?}",
			cnt,
			self.remaining(),
		);

		while cnt > 0 {
			// if advancing into next Bytes
			if cnt >= self.current_remaining() {
				cnt -= self.advance_to_next();
			} else {
				self.advance_current(cnt);
				cnt = 0;
			}
		}
	}
}