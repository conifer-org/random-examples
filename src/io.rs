//! This is `atom1`

use std::ops::Deref;
use std::slice;
use std::str;
use std::sync::atomic::{AtomicU32, Ordering};
use serde::{Serialize, Deserialize};

use postcard::{from_bytes, to_allocvec};

static HELLO: &'static str = "Hello, World!";


/// This is a stub function that invokes the extern function `print_str` with serialized arguments
/// > This function is autogenerated by CSL
fn print_str_stub(out_str: &str) {
    //the following function call with compressed parameters should be generated by Hydro
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct PrintStrArgs {
        string_to_print: String
    }
    let args_vec = to_allocvec(&PrintStrArgs {string_to_print: out_str.to_string()}).unwrap();


    //// The call to CSL/CSF that is statically/dynamically lined. One is chosen based on features enabled
    {
        #[cfg(feature = "csl_static")]
        csl::print_str(args_vec.as_ptr(), args_vec.len());

        //#[cfg(feature = "csl_dynamic")]
        // This DOES NOT WORK. The function call will fail due to impossible memory linking
        //csl::print_str(args_vec.as_ptr(), args_vec.len());
    }
}

/// This is a stub function that invokes the extern function `print_str` with serialized arguments
/// > This function is autogenerated by CSL
fn spawn_stub(electron_name: &str) {
    //the following function call with compressed parameters should be generated by Hydro
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct SpawnArgs {
        electron_name: String
    }
    let args_vec = to_allocvec(&SpawnArgs {electron_name: electron_name.to_string()}).unwrap();


    //// The call to CSL/CSF that is statically/dynamically lined. One is chosen based on features enabled
    {
        #[cfg(feature = "csl_static")]
        csl::spawn(args_vec.as_ptr(), args_vec.len());

        //#[cfg(feature = "csl_dynamic")]
        // This DOES NOT WORK. The function call will fail due to impossible memory linking
        // csl::spawn(args_vec.as_ptr(), args_vec.len());
    }
}

/// This is a stub function that invokes the extern function `get_args` with serialized arguments
/// > This function is autogenerated by CSL
fn get_args_stub(len: i32) -> Vec<u8> {
    let mut args_vec = vec![0 as u8; len as usize];

    //// The call to CSL/CSF that is statically/dynamically lined. One is chosen based on features enabled
    {
        #[cfg(feature = "csl_static")]
        csl::get_args(args_vec.as_ptr(), args_vec.len());

        //#[cfg(feature = "csl_dynamic")]
        // This DOES NOT WORK. The function call will fail due to impossible memory linking
        //csl::get_args(args_vec.as_ptr(), args_vec.len());
    }

    args_vec
}

/// This is a stub function that invokes the extern function `get_args` with serialized arguments
/// For the function `print_to_conterm`
/// > This function is autogenerated by CSL
fn get_args_print_to_conterm_stub(len: i32) -> String {
    let args_vec = get_args_stub(len);

    //the following function call with compressed parameters should be generated by Hydro
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct PrintToContermArgs {
        string_from_host: String
    }
    let args: PrintToContermArgs = from_bytes(args_vec.deref()).unwrap();
    args.string_from_host
}

/// This is a stub function that invokes the extern function `get_args` with serialized arguments
/// For the function `test_spawn`
/// > This function is autogenerated by CSL
fn get_args_test_spawn_stub(len: i32) -> u8 {
    let mut args_vec = get_args_stub(len);

    //the following function call with compressed parameters should be generated by Hydro
    #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
    struct TestSpawnArgs {
        thread_nos: u8,
    }
    let args: TestSpawnArgs = from_bytes(args_vec.deref()).unwrap();
    args.thread_nos
}



/// Prints a customized line to conterm
/// > This function is created by app developer
fn print_to_conterm(string_from_host: &str) {
    let mut id;

    //// The call to CSL/CSF that is statically/dynamically lined. One is chosen based on features enabled
    {
        #[cfg(feature = "csl_static")]
        { id = csl::get_id_static(); }

        #[cfg(feature = "csl_dynamic")]
        // This WILL WORK. The function call will succeed due to no memory linking requirement
        { id = csl::get_id_dynamic(); }

        #[cfg(feature = "csf_static__csl_static")]
        { id = csf::get_id(); }

        #[cfg(feature = "csf_dynamic__csl_static")]
        { id = csf::get_id(); }
    }

    let (csf, csl) = match id {
        60 => ("NA", "static"),
        65 => ("NA", "dynamic"),
        15 => ("static", "static"),
        20 => ("dynamic", "static"),
        _ => ("NA", "NA")
    };

    let out_str = format!("Message from app:: {}\nStatus: CSF({}), CSL({})",
                          string_from_host, csf, csl);

    // Stub is placed instead of real function by CSL
    print_str_stub(&out_str);
}


/// Stub function to invoke `print_to_conterm`
/// It's invoked from the outside **without** arguments
/// > This function is autogenerated by CSL
#[no_mangle]
pub extern "C" fn print_to_conterm_no_args_stub(_: i32) {
    print_to_conterm("I am Conifer!");
}

/// Stub function to invoke `print_to_conterm`
/// It's invoked from the outside **with** arguments
/// > This function is autogenerated by CSL
#[no_mangle]
pub extern "C" fn print_to_conterm_args_stub(len: i32) {

    let string_from_host; //this was initially an argument to this function, but CSL makes it
    // into local variable at top of the function

    // Stub is placed instead of real function by CSL
    string_from_host = get_args_print_to_conterm_stub(len);

    print_to_conterm(&string_from_host);
}



/// The function spawns `thread_nos` number of threads running the `print_to_conterm` function
/// > This function is created by app developer
fn test_spawn(thread_nos: u8) {
    for _ in 0..thread_nos {
        // Stub is placed instead of real function by CSL
        spawn_stub("print_to_conterm_no_args_stub");
    }
}

/// Stub function to invoke `test_spawn`
/// It's invoked from the outside **with** arguments
/// > This function is autogenerated by CSL
#[no_mangle]
pub extern "C" fn test_spawn_args_stub(len: i32) {

    let thread_nos; //this was initially an argument to this function, but Hydro makes it
    // into local variable at top of the function

    // Stub is placed instead of real function by CSL
    thread_nos = get_args_test_spawn_stub(len);

    test_spawn(thread_nos);
}

/// Stub function to invoke `test_spawn`
/// It's invoked from the outside **without** arguments
/// > This function is autogenerated by CSL
#[no_mangle]
pub extern "C" fn test_spawn_no_args_stub(_: i32) {
    static FIXED: u8 = 4;
    test_spawn(FIXED);
}

/// Stub function to run some test code for Multi-MoleculeThreads TEMPORARY
#[no_mangle]
pub extern "C" fn multi_moleculethreads_test_no_args_stub(_: i32) {
    print_to_conterm("Starting Thread 1");
    spawn_stub("print_numbers_loop_no_args_stub");
    print_to_conterm("Starting Thread 2");
    spawn_stub("print_numbers_loop_no_args_stub");
}

static THE_NUM: AtomicU32 = AtomicU32::new(0);
static LOOP_ID: AtomicU32 = AtomicU32::new(0);

/// Stub function to run some test code for Print numbers in a loop TEMPORARY
#[no_mangle]
pub extern "C" fn print_numbers_loop_no_args_stub(_: i32) {
    let loop_id = LOOP_ID.fetch_add(1, Ordering::SeqCst);
    for _ in 0..50 {
        print_to_conterm(&format!("Loop ID {} num: {}", loop_id, THE_NUM.fetch_add(1, Ordering::SeqCst)));
    }
}
