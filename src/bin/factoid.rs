use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/factoidsd").unwrap_or_else(|_err| {
        eprintln!("Couldn't connect to factoids daemon. Is it running?");
        std::process::exit(1);
    });

    stream.write_all(b"get test\n").unwrap();
    println!("Sent request.");

    let _ = stream.shutdown(std::net::Shutdown::Read);

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    println!("{}", response);

    let _ = stream.shutdown(std::net::Shutdown::Both);

    Ok(())
}