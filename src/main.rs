#[cfg(not(target_os = "linux"))]
compile_error!("rust-ds-lab supports Linux only.");

use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn respond(stream: &mut TcpStream, status: &str, body: &str) -> io::Result<()> {
    write!(stream, "HTTP/1.1 {status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nContent-Security-Policy: default-src 'none'; style-src 'unsafe-inline'; form-action 'self'; base-uri 'none'; frame-ancestors 'none'\r\n\r\n{body}", body.len())
}

fn handle(mut stream: TcpStream) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    let mut reader = BufReader::new(&stream);
    // Bound the complete header, not merely the first read.
    let mut header = Vec::new();
    loop {
        let available = reader.fill_buf()?;
        if available.is_empty() {
            return Ok(());
        }
        let mut consumed = 0;
        for byte in available {
            header.push(*byte);
            consumed += 1;
            if header.ends_with(b"\r\n\r\n") || header.len() > 16384 {
                break;
            }
        }
        reader.consume(consumed);
        if header.len() > 16384 {
            return respond(
                &mut stream,
                "431 Request Header Fields Too Large",
                "Request too large",
            );
        }
        if header.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    let request = String::from_utf8_lossy(&header);
    let mut parts = request.lines().next().unwrap_or("").split_whitespace();
    if parts.next() != Some("GET") {
        return respond(
            &mut stream,
            "405 Method Not Allowed",
            "Only GET is supported",
        );
    }
    let target = parts.next().unwrap_or("/");
    if target == "/health" {
        return respond(&mut stream, "200 OK", "ok");
    }
    if target.split('?').next() != Some("/") {
        return respond(&mut stream, "404 Not Found", "Not found");
    }
    let page = rust_ds_lab::web::page(target.split_once('?').map_or("", |(_, q)| q));
    respond(&mut stream, "200 OK", &page)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args == ["--help"] || args == ["-h"] {
        println!("rust-ds-lab [--port PORT]\nLinux data structure visualizer. Open the printed local URL. Ctrl+C stops it.");
        return Ok(());
    }
    let port: u16 = match args.as_slice() {
        [] => 7878,
        [flag, value] if flag == "--port" => {
            value.parse().ok().filter(|p| *p != 0).ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "PORT must be 1..65535")
            })?
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Usage: rust-ds-lab [--port PORT]",
            ))
        }
    };
    let listener = TcpListener::bind(("127.0.0.1", port))?;
    println!("Rust DS Lab → http://127.0.0.1:{port}\nOpen this URL in your Linux browser. Stop with Ctrl+C.");
    // A deliberately small, sequential localhost teaching server. No remote deployment.
    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                if let Err(error) = handle(stream) {
                    eprintln!("Request: {error}");
                }
            }
            Err(error) => eprintln!("Connection: {error}"),
        }
    }
    Ok(())
}
