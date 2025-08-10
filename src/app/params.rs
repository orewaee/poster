use std::path::PathBuf;

#[derive(Debug)]
pub struct HttpParams {
    pub host: String,
    pub port: u16,
    pub static_path: PathBuf,
}

impl Default for HttpParams {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: 2201,
            static_path: PathBuf::from("static"),
        }
    }
}

pub struct HttpParamsBuilder {
    params: HttpParams,
}

pub enum HttpParamsError {
    InvalidHost,
    InvalidPort,
}

impl HttpParamsBuilder {
    pub fn new() -> Self {
        Self {
            params: HttpParams::default(),
        }
    }

    pub fn host(mut self, host: String) -> Result<Self, HttpParamsError> {
        if host.is_empty() {
            return Err(HttpParamsError::InvalidHost);
        }

        self.params.host = host;
        Ok(self)
    }

    pub fn port(mut self, port: u16) -> Result<Self, HttpParamsError> {
        if port == 0 {
            return Err(HttpParamsError::InvalidPort);
        }

        self.params.port = port;
        Ok(self)
    }

    pub fn static_path(mut self, path: PathBuf) -> Result<Self, HttpParamsError> {
        self.params.static_path = path;
        Ok(self)
    }
}
