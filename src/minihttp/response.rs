use std::fmt::{self, Write, Debug};

use bytes::{BytesMut, BufMut};

use minihttp::date;

pub struct Response {
    headers: Vec<(String, String)>,
    response: String,
    status_message: StatusMessage,
}

enum StatusMessage {
    Ok,
    Custom(u32, String)
}

impl Response {
    pub fn new() -> Response {
        Response {
            headers: Vec::new(),
            response: String::new(),
            status_message: StatusMessage::Ok,
        }
    }

    pub fn status_code(&mut self, code: u32, message: &str) -> &mut Response {
        self.status_message = StatusMessage::Custom(code, message.to_string());
        self
    }

    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        self.headers.push((name.to_string(), val.to_string()));
        self
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response = s.to_string();
        self
    }
}

impl Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<HTTP Response {} {}>", self.status_message, self.response)
    }
}

pub fn encode(msg: Response, buf: &mut BytesMut) {
    let length = msg.response.len();
    let now = date::now();

    write!(buf, "\
        HTTP/1.1 {}\r\n\
        Server: Example\r\n\
        Content-Length: {}\r\n\
        Date: {}\r\n\
    ", msg.status_message, length, now).unwrap();

    for &(ref k, ref v) in &msg.headers {
        push(buf, k.as_bytes());
        push(buf, ": ".as_bytes());
        push(buf, v.as_bytes());
        push(buf, "\r\n".as_bytes());
    }

    push(buf, "\r\n".as_bytes());
    push(buf, msg.response.as_bytes());
}

fn push(buf: &mut BytesMut, data: &[u8]) {
    buf.reserve(data.len());
    unsafe {
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}

impl fmt::Display for StatusMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StatusMessage::Ok => f.pad("200 OK"),
            StatusMessage::Custom(c, ref s) => write!(f, "{} {}", c, s),
        }
    }
}
