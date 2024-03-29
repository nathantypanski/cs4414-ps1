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
                        Some(pn) => {
                            println!("Received connection from: [{:s}]",
                                    pn.to_str());
                            unsafe { 
                                atomic_add(&mut visitor_count, 1);
                                println!("Visitor count: {:u}", visitor_count);
                            }
                            },
                        None => ()
                    }
                },
                None => ()
            }
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            let filename = get_requested_filename(request_str);
            if (filename == "") {
let response: ~str = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
<doctype !html><html><head><title>Hello, Rust!</title>
<style>body { background-color: #111; color: #FFEEAA }
h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
</style></head>
<body>
<h1>Greetings, Krusty!</h1>
<h2>Visitor count: "
+ unsafe{
    visitor_count.to_str()
} + "</h2>
</body></html>\r\n";
            stream.write(response.as_bytes());
            }
            else {
                println!("filename: {:s}", filename);
                println!("Received request :\n{:s}", request_str);
                let response = read_filename(filename);
                stream.write(response.as_bytes());
            }
            println!("Connection terminates.");
        }
    }
}

fn get_requested_filename(request: &str) -> &str {
    let lines: ~[&str] = request.split('\n').collect(); 
    let firstline      = lines[0];
    let after          = firstline.slice_from(4).to_owned();
    let then: ~[&str]  = after.split(' ').collect();
    let filename       = then[0].slice_from(1);

    filename
}

fn read_filename(filename : &str) -> ~str {
    let mut contents = ~"";
    if (filename.slice_from(filename.len() - 5) != ".html") {
        contents = fmt_bad_response() + "\nWanted: " + filename;
    }
    else {
        let filepath= Path::new(filename.clone());
        match File::open_mode(&filepath, Open, Read) {
            Some (file) => {
                println("File opened for reading ...");
                contents = fmt_response(get_file_contents(file));
            },
            _ => {
                println!("Error opening file: {:s}", filename);
            }
        }
    }
    contents
}

fn get_file_contents(file: File) -> ~str {
    let mut filereader = ~BufferedReader::new(file);
    println("Reading file ...");
    let contents: ~[u8] = filereader.read_to_end();
    str::from_utf8(contents).to_owned()
}

fn fmt_response(file_contents : &str) -> ~str {
    let response = ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n"
    + file_contents;
    response
}

fn fmt_bad_response() -> ~str {
    let response = ~"HTTP/1.1 403 OK\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\n"
    + "403 Bad filetype.";
    response
}

fn atomic_add(var: &mut uint, count: uint) -> uint {
    let oldval = var.clone();
    *var = *var + count;
    return oldval;
}
