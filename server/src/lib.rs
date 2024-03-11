mod http;
mod thread_pool;
mod worker;

use std::{
    io::{BufRead, Read},
    net::{TcpListener, TcpStream},
};

use http::Request;

use crate::thread_pool::ThreadPool;

pub struct WebServer {}

impl WebServer {
    pub fn start() -> std::io::Result<()> {
        let thread_pool = ThreadPool::new(4);
        let socket = TcpListener::bind("127.0.0.1:3000")?;
        println!("-> Local: http://localhost:3000");

        for stream in socket.incoming() {
            let stream = stream.expect("Fail on stream");
            thread_pool.execute(|| handle_stream(stream))
        }

        Ok(())
    }
}

fn handle_stream(mut stream: TcpStream) {
    let request = match Request::new(&mut stream) {
        Ok(r) => r,
        Err(e) => {
            println!("Error on create request {}", e);
            return;
        }
    };

    println!("Request {:?}", request);
}
