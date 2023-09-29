use std::thread;

use bytes::{Bytes, Buf};
use bitvec::prelude::*;
use flume::{Receiver, Sender};

use super::frame::Frame;

mod search_arr;
