use std::{io, slice, str};
use std::fmt::{self, Write, Debug};

use bytes::{BytesMut, BufMut};

use httparse;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: u8,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Request {
    pub fn method(&self) -> &str {
        &*self.method
    }

    pub fn path(&self) -> &str {
        &*self.path
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn headers(&self) -> RequestHeaders {
        RequestHeaders {
            headers: self.headers.iter(),
            req: self,
        }
    }

    pub fn body(&self) -> &str {
        &*self.body
    }

}

impl fmt::Debug for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<HTTP Request {} {}>", self.method(), self.path())
    }
}

pub fn decode(buf: &mut BytesMut) -> io::Result<Option<Request>> {
    let (method, path, version, headers, amt) = {
        // XXX: We only support up to 16 headers.
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut r = httparse::Request::new(&mut headers);
        let status = try!(r.parse(buf).map_err(|e| {
            let msg = format!("failed to parse http request: {:?}", e);
            io::Error::new(io::ErrorKind::Other, msg)
        }));

        let amt = match status {
            // We have read all of the data...
            httparse::Status::Complete(amt) => amt,
            // We need to read more data...
            httparse::Status::Partial => return Ok(None),
        };

        (r.method.unwrap().to_owned(),
         r.path.unwrap().to_owned(),
         r.version.unwrap(),
         r.headers
          .iter()
          .map(|h| (h.name.to_owned(), String::from_utf8(h.value.to_vec()).unwrap()))
          .collect(),
         amt)
    };

    // Consume the buffer that we copied the header data out of.
    buf.split_to(amt);

    Ok(Request {
        method: method,
        path: path,
        version: version,
        headers: headers,
        // Consume the rest of the buffer as the request body.
        body: String::from_utf8(buf.take().to_vec()).unwrap(),
    }.into())
}

pub fn encode(request: Request, buf: &mut BytesMut) {
    write!(buf, "\
        {} {} HTTP/1.1\r\n\
        ", request.method, request.path).unwrap();
    for &(ref k, ref v) in &request.headers {
        push(buf, k.as_bytes());
        push(buf, ": ".as_bytes());
        push(buf, v.as_bytes());
        push(buf, "\r\n".as_bytes());
    }
    push(buf, "\r\n".as_bytes());
    push(buf, request.body.as_bytes());
}


fn push(buf: &mut BytesMut, data: &[u8]) {
    buf.reserve(data.len());
    unsafe {
        buf.bytes_mut()[..data.len()].copy_from_slice(data);
        buf.advance_mut(data.len());
    }
}

pub struct RequestHeaders<'req> {
    headers: slice::Iter<'req, (String, String)>,
    req: &'req Request,
}

impl<'req> Iterator for RequestHeaders<'req> {

    type Item = (&'req String, &'req String);

    fn next(&mut self) -> Option<(&'req String, &'req String)> {
        self.headers.next().map(|&(ref a, ref b)| {
            (&*a, &*b)
        })
    }
}

#[test]
fn test_request_encode() {
    let mut buf = BytesMut::with_capacity(1024);
    let request = Request {
        method: "GET".to_string(),
        path: "/".to_string(),
        version: 1,
        headers: vec![
            ("User-Agent".to_string(), "rust".to_string()),
            ("Accepts".to_string(), "text/html".to_string()),
        ],
        body: "".to_string(),
    };
    encode(request, &mut buf);
    assert!(String::from_utf8(buf.to_vec()).unwrap() == "GET / HTTP/1.1\r\nUser-Agent: rust\r\nAccepts: text/html\r\n\r\n");
}