//! This is `atom1`

use std::ops::Deref;
use std::slice;
use std::str;
use serde::{Serialize, Deserialize};

use postcard::{from_bytes, to_allocvec};

//"test" is the name of the wasm module; equivalent to the atom name; in this case it refers to spinner imports
#[link(wasm_import_module = "test")]
extern "C" {
    fn get_args(ptr: *const u8, len: usize);
    fn print_str(ptr: *const u8, len: usize);
    fn spawn_dup();
}

// Define a string that is accessible within the wasm
// linear memory.
static HELLO: &'static str = "Hello, World!";

#[no_mangle]
pub extern "C" fn print_to_conterm(len: i32) {

    let string_from_host; //this was initially an argument to this function, but Hydro makes it
    // into local variable at top of the function
    unsafe {
        let mut args_vec = vec![0 as u8; len as usize];
        get_args(args_vec.as_ptr(), args_vec.len());

        //the following function call with compressed parameters should be generated by Hydro
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct PrintToContermArgs {
            string_from_host: String
        }
        let args: PrintToContermArgs = from_bytes(args_vec.deref()).unwrap();
        string_from_host = args.string_from_host;
    }

    let out_str = format!("Printing to Conterm FROM wasm:: {} {}", string_from_host, csl::add(1,5));

    unsafe {

        //the following function call with compressed parameters should be generated by Hydro
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct PrintStrArgs {
            string_to_print: String
        }
        let args_vec = to_allocvec(&PrintStrArgs {string_to_print: out_str}).unwrap();

        print_str(args_vec.as_ptr(), args_vec.len());

    }
}

#[no_mangle]
pub extern "C" fn test_spawn(len: i32) {

    let thread_nos; //this was initially an argument to this function, but Hydro makes it
    // into local variable at top of the function
    unsafe {
        let mut args_vec = vec![0 as u8; len as usize];
        get_args(args_vec.as_ptr(), args_vec.len());

        //the following function call with compressed parameters should be generated by Hydro
        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct TestSpawnArgs {
            thread_nos: u8,
        }
        let args: TestSpawnArgs = from_bytes(args_vec.deref()).unwrap();
        thread_nos = args.thread_nos;
    }

    for _ in 0..thread_nos {
        unsafe {
            spawn_dup();
        }
    }
}
