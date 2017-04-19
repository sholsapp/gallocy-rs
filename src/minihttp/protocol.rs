use std::io;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::{ServerProto, ClientProto};

use minihttp::request::{self, Request};
use minihttp::response::{self, Response};

pub struct HttpCodec;

/// Implements a HTTP protocol decoder.
///
impl Decoder for HttpCodec {
    type Item = Request;
    type Error = io::Error;
    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        request::decode(buf)
    }
}

/// Implements a HTTP protocol encoder.
///
impl Encoder for HttpCodec {
    type Item = Response;
    type Error = io::Error;
    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        response::encode(msg, buf);
        Ok(())
    }
}

pub struct Http;

/// Implememnts a server-side HTTP protocol.
///
impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Http {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = io::Result<Framed<T, HttpCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpCodec>> {
        Ok(io.framed(HttpCodec))
    }
}

/// Implements a client-side HTTP protocol.
///
impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for Http {
    // XXX: Fix me.
    type Request = Response;
    // XXX: Fix me.
    type Response = Request;
    type Transport = Framed<T, HttpCodec>;
    type BindTransport = io::Result<Framed<T, HttpCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpCodec>> {
        Ok(io.framed(HttpCodec))
    }
}