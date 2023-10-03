use std::thread;

use bytes::Bytes;
use bitvec::prelude::*;
use flume::{Receiver, Sender};

use crate::SEND_EXPECT_MSG;

/// bitshift an entire stream of data to try to fuck up the rest of the code
#[derive(Debug)]
pub struct Task {
	rx: Receiver<Bytes>,
	tx: Sender<u8>,
	shift_by: usize,
	noise: u8
}

impl Task {
	const NAME: &str = "stream bit shift";

	/// setup the state for this task and build the thread
	pub fn new(rx: Receiver<Bytes>, shift_by: u8) -> (Self, Receiver<u8>) {
		let (output_tx, output_rx) = flume::unbounded(); // should this be bounded?

		(
			Self {
				rx,
				tx: output_tx,
				shift_by: shift_by as usize,
				noise: 0b01010010
			},
			output_rx
		)
	}

	/// starts the thread for the task
	pub fn start(mut self) {
		thread::Builder::new().name(Self::NAME.to_string()).spawn(move || {
			while let Ok(bin) = self.rx.recv() {
				let mut bin = bin.to_vec();
				let view = bin.view_bits_mut::<Msb0>();

				view.shift_right(self.shift_by);


				for b in bin {
					self.tx.send(b ^ self.noise).expect(SEND_EXPECT_MSG);
					self.noise <<= 1;
				}
			}
		}).expect(Self::NAME);
	}
}
