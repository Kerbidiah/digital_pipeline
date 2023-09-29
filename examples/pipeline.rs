const NUM_BYTES: usize = 1_000;
const SIZE_FRAME: usize = 100;
const NUM_FRAMES: usize = NUM_BYTES / SIZE_FRAME;

use digital_pipeline::prelude::*;
use digital_pipeline::middle_man;
use rand::prelude::*;
use bytes::Bytes;

use std::thread;
use std::time::Duration;

fn main() {
	// setup the tasks of the pipeline
	let (tx_start, rx_start) = create_bytes_channel();
	let (encoder, rx_encoder) = encode_task::Task::new(rx_start.clone());
	let (middle, rx_middle_man) = middle_man::Task::new(rx_encoder.clone()); // this is for testing purposes
	let (searcher, rx_search) = search_task::Task::new(rx_middle_man.clone());
	let (decoder, rx_decode) = decode_task::Task::new(rx_search.clone());

	let mut original_data = Vec::with_capacity(NUM_FRAMES);
	for _ in 0..NUM_FRAMES {
		original_data.push(random_bytes(SIZE_FRAME));
	}

	// start the tasks (any order works, but doing it in reverse is probably best)
	decoder.start();
	searcher.start();
	middle.start();
	encoder.start();

	// send the data into the pipeline
	for each in &original_data {
		tx_start.send(each.clone()).unwrap();
	}
	drop(tx_start);

	// progress monitoring thread
	let rx_decode_clone = rx_decode.clone();
	let info_thread = thread::spawn(move || {
		loop {
			dbg!(rx_start.len());
			dbg!(rx_encoder.len());
			dbg!(rx_middle_man.len());
			dbg!(rx_search.len());
			dbg!(rx_decode_clone.len());
			dbg!("---------");

			thread::sleep(Duration::from_millis(500));
		}
	});

	// receive the data from the pipeline
	let mut output_data: Vec<Bytes> = Vec::new();
	for i in 0..NUM_FRAMES {
		dbg!(i);
		output_data.push(rx_decode.recv().unwrap());
	}
	dbg!("DONE");

	for (i, (og, out)) in original_data.iter().zip(output_data).enumerate() {
		dbg!(i);
		assert_eq!(og.to_vec(), out.to_vec())
	}

	info_thread.join().unwrap();
}

fn random_bytes(len: usize) -> Bytes {
	let mut rng = rand::thread_rng();
	let mut data: Vec<u8> = vec![0; len];
	rng.fill_bytes(&mut data);

	data.into()
}
