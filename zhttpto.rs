//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str};
use std::io::buffered::BufferedReader;

static IP: &'static str  = "127.0.0.1";
static PORT:        int  = 4414;
static mut visitor_count: uint = 0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            let lines: ~[&str] = request_str.split('\n').collect(); 
            let firstline = lines[0];
            let after=firstline.slice_from(4).to_owned();
            let then: ~[&str]=after.split(' ').collect();
            let filename = then[0].slice_from(1);
            println!("filename: {:s}", filename);
            println!("Received request :\n{:s}", request_str);
            unsafe { atomic_add(&mut visitor_count, 1);
                     println!("Visitor count: {:u}", visitor_count);
            }
            let filepath = Path::new(filename);
            match File::open(&filepath) {
                Some (file) => {
                    let mut filereader = ~BufferedReader::new(file);
                    let mut contents : ~str = ~"";
                    println("Reading file ...");
                    for line in filereader.lines() {
                        println!("Found line: {:s}.", line);
                        contents = contents + "<br />" + line;
                    }
                    let response: ~str = 
                        ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                        <doctype !html><html><head><title>Hello, Rust!</title></head>
                        <body>"
                        + contents
                            + unsafe{
                                visitor_count.to_str()
                            } + "</h2>
                        </body></html>\r\n";
                    stream.write(response.as_bytes());
                    println("Connection terminates.");
                }
                None => {
                    let response: ~str = 
                        ~"HTTP/1.1 404 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                        <doctype !html><html><head><title>Hello, Rust!</title></head>
                        <body>
                        </body></html>\r\n";
                    stream.write(response.as_bytes());
                    println!("Mad world.");
                }
            }
        }
    }
}

fn atomic_add(var: &mut uint, count: uint) -> uint {
    let oldval = var.clone();
    *var = *var + count;
    return oldval;
}
