use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Method::GET => write!(f, "GET"),
        }
    }
}

pub enum HttpVersion {
    ONE = 1,
    TWO = 2,
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            HttpVersion::ONE => write!(f, "HTTP/1.1"),
            HttpVersion::TWO => write!(f, "HTTP/2"),
        }
    }
}

pub struct Request {
    pub method: Method,
    path: String,
    version: HttpVersion,
    headers: Vec<(String, String)>,
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {} {}", self.method, self.path, self.version)?;
        for (k, v) in &self.headers {
            write!(f, "\r\n{}: {}", k, v)?;
        }
        Ok(())
    }
}

impl Request {
    fn default() -> Request {
        Request {
            method: Method::GET, 
            path: "".to_string(), 
            version: HttpVersion::ONE, 
            headers: vec![],
        }
    }

    pub fn parse(_: Cow<'_, str>) -> Request {
        let request = Request::default();
        return request
    }
}
