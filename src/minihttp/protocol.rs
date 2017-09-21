use std::io;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::{ServerProto, ClientProto};

use minihttp::request::{self, Request};
use minihttp::response::{self, Response};

pub struct HttpServerCodec;

/// Implements a HTTP protocol decoder.
///
impl Decoder for HttpServerCodec {
    type Item = Request;
    type Error = io::Error;
    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        request::decode(buf)
    }
}

/// Implements a HTTP protocol encoder.
///
impl Encoder for HttpServerCodec {
    type Item = Response;
    type Error = io::Error;
    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        response::encode(msg, buf);
        Ok(())
    }
}

pub struct HttpClientCodec;

impl Decoder for HttpClientCodec {
    type Item = Response;
    type Error = io::Error;
    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Response>> {
        response::decode(buf)
    }
}

impl Encoder for HttpClientCodec {
    type Item = Request;
    type Error = io::Error;
    fn encode(&mut self, msg: Request, buf: &mut BytesMut) -> io::Result<()> {
        request::encode(msg, buf);
        Ok(())
    }
}

pub struct Http;

/// Implememnts a server-side HTTP protocol.
///
impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for Http {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpServerCodec>;
    type BindTransport = io::Result<Framed<T, HttpServerCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpServerCodec>> {
        Ok(io.framed(HttpServerCodec))
    }
}

/// Implements a client-side HTTP protocol.
///
impl<T: AsyncRead + AsyncWrite + 'static> ClientProto<T> for Http {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, HttpClientCodec>;
    type BindTransport = io::Result<Framed<T, HttpClientCodec>>;

    fn bind_transport(&self, io: T) -> io::Result<Framed<T, HttpClientCodec>> {
        Ok(io.framed(HttpClientCodec))
    }
}
