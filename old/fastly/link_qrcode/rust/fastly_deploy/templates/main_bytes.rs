// NAME of the module to insert
mod qr;

#[macro_use]
extern crate lazy_static;
// IMPORT HERE
use qr::*;
use rand::RngCore;
use std::sync::Mutex;
use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::{Hash, Hasher};



lazy_static! {
    static ref DISPATCHER_OPTIONS: Mutex<Vec<i32>> = Mutex::new(vec![]);
}

#[warn(non_snake_case)]
#[no_mangle]
pub fn discriminate(total: i32) -> i32 {
    
    let popId = (rand::thread_rng().next_u32()%1000000u32) as i32;
    let r = popId%total;
    DISPATCHER_OPTIONS.lock().unwrap().push(r);
    r
}

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.

use fastly::{Error, Request, Response};
use fastly::http::{StatusCode};

#[fastly::main]
fn main(_req: Request) -> Result<Response, Error> {


	let pop = match std::env::var("FASTLY_POP") {
		Ok(val) => val,
		Err(_) => "NO_POP".to_string()
	};
    let now = std::time::Instant::now();
    let img = unsafe { run_qr(String::from("Hello world form MEWE !")) };
    let lapsed = now.elapsed().as_nanos();
    let path = DISPATCHER_OPTIONS.lock().unwrap();

    
    let res = Response::from_body("[]")
    .with_header("Xtime", format!("{:?}", now.elapsed().as_nanos()))
    .with_header("Xpop", format!("{:?}", pop))
    .with_header("Xdispatcher", format!("{:?}", path))
    //.with_header("XContent", img)
    .with_status(StatusCode::OK);
    Ok(res)
}

//ebeeeea1b8c6ceabd5a4f0c80c68ccf312267b3e4d5488442bea9df52f7d3f00cab6dbd45b7e5f884a4895000e94010441ea9070aed86b97f0fe087d19e5f353
//66aa39821189f2cce1ffdad44f3aa60dd276599505f920581350150eda5429d44687a2309d69975ddd6e67c423b91954d0ae3bdb33b19816041716b7c7ef2045


