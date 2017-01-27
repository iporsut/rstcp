use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, BufRead, BufReader};
use std::thread;

fn handle_client(stream: TcpStream) -> io::Result<()> {
    write!(&stream, "Enter your name: ")?;
    let mut f = BufReader::new(&stream);
    let mut buffer = String::new();
    f.read_line(&mut buffer)?;
    writeln!(&stream, "Hello, {}!", buffer.trim())?;

    Ok(())
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream).unwrap();
                });
            }
            Err(_) => {
            }
        }
    }
}
