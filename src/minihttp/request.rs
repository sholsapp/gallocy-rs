use std::{io, slice, str, fmt};

use bytes::BytesMut;

use httparse;

pub struct Request {
    method: String,
    path: String,
    version: u8,
    headers: Vec<(String, String)>,
    body: String,
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
    // TODO: we should grow this headers array if parsing fails and asks
    //       for more headers
    let (method, path, version, headers, amt) = {
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
        body: String::from_utf8(buf.take().to_vec()).unwrap(),
    }.into())
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
