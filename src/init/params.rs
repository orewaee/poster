use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct InitParams {
    pub static_path: PathBuf,
    pub database_path: PathBuf,
}

impl Default for InitParams {
    fn default() -> Self {
        Self {
            static_path: PathBuf::from("static"),
            database_path: PathBuf::from("poster.db"),
        }
    }
}

pub struct InitParamsBuilder {
    params: InitParams,
}

#[derive(Debug)]
pub enum InitParamsError {}

impl InitParamsBuilder {
    pub fn new() -> Self {
        Self {
            params: InitParams::default(),
        }
    }

    pub fn static_path(&mut self, path: PathBuf) -> Result<&mut Self, InitParamsError> {
        self.params.static_path = path;
        Ok(self)
    }

    pub fn database_path(&mut self, path: PathBuf) -> Result<&mut Self, InitParamsError> {
        self.params.database_path = path;
        Ok(self)
    }

    pub fn build(&self) -> Result<InitParams, InitParamsError> {
        Ok(self.params.clone())
    }
}
