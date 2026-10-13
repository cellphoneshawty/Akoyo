use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

/// Checks whether the program on `port` is Akoyo, so launching Akoyo a second
/// time just opens the browser instead of failing.
pub fn is_akoyo_running(port: u16) -> bool {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let Ok(mut stream) = TcpStream::connect_timeout(&addr, Duration::from_secs(2)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
    if stream
        .write_all(b"GET /api/status HTTP/1.0\r\nHost: localhost\r\n\r\n")
        .is_err()
    {
        return false;
    }

    let mut response = Vec::new();
    let _ = stream.take(64 * 1024).read_to_end(&mut response);
    String::from_utf8_lossy(&response).contains(r#""app":"akoyo""#)
}
