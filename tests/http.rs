use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::{Child, Command, Stdio},
    time::Duration,
};
struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn request(port: u16, request: &[u8]) -> String {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(4)))
        .unwrap();
    stream.write_all(request).unwrap();
    let mut result = String::new();
    stream.read_to_string(&mut result).unwrap();
    result
}
#[test]
fn server_serves_pages_and_bounds_requests() {
    let reservation = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = reservation.local_addr().unwrap().port();
    drop(reservation);
    let child = Command::new(env!("CARGO_BIN_EXE_rust-ds-lab"))
        .args(["--port", &port.to_string()])
        .stdout(Stdio::null())
        .spawn()
        .unwrap();
    let _server = Server(child);
    let mut ready = false;
    for _ in 0..50 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            ready = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(ready, "server failed to start");
    let page = request(
        port,
        b"GET /?topic=heap HTTP/1.1\r\nHost: localhost\r\n\r\n",
    );
    assert!(page.starts_with("HTTP/1.1 200"));
    assert!(page.contains("Min heap"));
    assert!(page.contains("Content-Security-Policy:"));
    let (head, body) = page.split_once("\r\n\r\n").unwrap();
    let length = head
        .lines()
        .find_map(|l| l.strip_prefix("Content-Length: "))
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert_eq!(length, body.len());
    assert!(request(port, b"POST / HTTP/1.1\r\n\r\n").starts_with("HTTP/1.1 405"));
    assert!(request(port, b"GET /missing HTTP/1.1\r\n\r\n").starts_with("HTTP/1.1 404"));
    assert!(request(port, b"GET /health HTTP/1.1\r\n\r\n").ends_with("ok"));
    let oversized = format!("GET / HTTP/1.1\r\nX-Large: {}\r\n\r\n", "x".repeat(16384));
    // The peer may reset when unconsumed request bytes remain; read just the response header.
    let mut stream = TcpStream::connect(("127.0.0.1", port)).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(3)))
        .unwrap();
    stream.write_all(oversized.as_bytes()).unwrap();
    // TCP reads may return only part of the status line.
    let mut status = [0; 12];
    stream.read_exact(&mut status).unwrap();
    assert_eq!(&status, b"HTTP/1.1 431");
}
