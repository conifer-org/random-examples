use std::any::Any;
use std::ops::Deref;
use std::slice;
use std::str;
use std::sync::atomic::{AtomicU32, Ordering};
use serde::{Serialize, Deserialize};

use postcard::{from_bytes, to_allocvec};

/// Let's enjoy doing some addions
/// THis is App-dev defined
#[atom(name = "multi_app_thread")]
#[boot_electron]
fn enjoy_adding() {
	let thread_nos = 5;
	for i in 0..thread_nos {
		// Stub is placed instead of real function by CSL
		let added = csl::csl_spawn_and_wait!(add_return_str(i, i + 1));
		csl::print(&added.1);
		//TODO: we need to spawn and join. Function name must change
		//csl::spawn::<()>("print_random_stuff", None);
	}
}

/// A developer-defined function that takes a String and "addes" it
#[atom(name = "multi_app_thread")]
#[electron(callers("multi_app_thread"))]
fn add_return_str(a: u32, b: u32) -> (u32, String) {
	let sum = a + b;
	(sum, format!("( ͡° ͜ʖ ͡°)_/¯ {a} + {b} = {}", a + b))
}
