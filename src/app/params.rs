use std::path::PathBuf;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
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

    pub fn host(&mut self, host: String) -> Result<&mut Self, HttpParamsError> {
        if host.is_empty() {
            return Err(HttpParamsError::InvalidHost);
        }

        self.params.host = host;
        Ok(self)
    }

    pub fn port(&mut self, port: u16) -> Result<&mut Self, HttpParamsError> {
        if port == 0 {
            return Err(HttpParamsError::InvalidPort);
        }

        self.params.port = port;
        Ok(self)
    }

    pub fn static_path(&mut self, path: PathBuf) -> Result<&mut Self, HttpParamsError> {
        self.params.static_path = path;
        Ok(self)
    }

    pub fn build(&self) -> Result<HttpParams, HttpParamsError> {
        Ok(self.params.clone())
    }
}
