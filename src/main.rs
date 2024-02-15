use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::{fs, thread};
use std::time::Duration;

use rustapi::simpleapi::simple_api;
use rustapi::ThreadPool;

fn main() {
    let listener =TcpListener::bind("127.0.0.1:8080").unwrap();

    let th_pool = ThreadPool::new(10); //Creates a 5 thread pool

    //Initializing our router map inside an Mutex so we can share it between threads
    let routes = Arc::new(Mutex::new(simple_api::Router::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //Creates or assigns a new thread to the request
        let routes_clone = Arc::clone(&routes);
        th_pool.execute(|| {
            simple_api::Request::handle_request(stream, routes_clone)
        });
    }
}
