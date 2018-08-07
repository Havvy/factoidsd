extern crate dirty_db;

use std::os::unix::net::{UnixStream, UnixListener};
use std::io::{Read, Write, Result as IoResult, BufRead, BufReader};

const DAEMON_SOCKET_LOCATION: &str = "/tmp/factoidsd";

fn main() -> IoResult<()> {
    std::fs::remove_file(DAEMON_SOCKET_LOCATION)?;
    let listener = UnixListener::bind(DAEMON_SOCKET_LOCATION)?;

    // accept connections and process them
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // connection succeeded
                handle_client(stream)
            }
            Err(_err) => {
                // connection failed
                continue;
            }
        }
    }

    // Should be unreachable.
    Ok(())
}


fn handle_client(mut stream: UnixStream) {
    println!("Got a client!");

    let read_buffer = BufReader::new(stream.try_clone().unwrap());

    for line_result in read_buffer.lines() {
        match line_result {
            Ok(line) -> { println!("{}", line); },
            Err(err) -> { eprintln!("{:?}", err); }
        }
        
        let _ = stream.write_all(b"blah");
    }

    let mut buffer = String::new();

    let _ = stream.read_to_string(&mut buffer);

    println!("{}", buffer);

    let _ = stream.write_all(b"blah");

    let _ = stream.shutdown(std::net::Shutdown::Both);

    println!("Handled client!");
}