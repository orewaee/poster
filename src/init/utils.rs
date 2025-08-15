use std::fs;

use crate::init::{error::InitError, params::InitParams};

const DEFAULT_STYLES: &'static str = include_str!("../../static/styles.css");
const DEFAULT_TEMPLATE: &'static str = include_str!("../../templates/post.html");

pub fn init(params: InitParams) -> Result<(), InitError> {
    fs::create_dir(&params.static_path)
        .map_err(|_| InitError::FailedToCreateDir)
        .expect("failed to create static dir");
    fs::create_dir("templates")
        .map_err(|_| InitError::FailedToCreateDir)
        .expect("failed to create templates dir");
    fs::create_dir("posts")
        .map_err(|_| InitError::FailedToCreateDir)
        .expect("failed to create posts dir");
    fs::write(params.static_path.join("styles.css"), DEFAULT_STYLES)
        .map_err(|_| InitError::FailedToCreateFile)
        .expect("failed to create default styles");
    fs::write("templates/post.html", DEFAULT_TEMPLATE)
        .map_err(|_| InitError::FailedToCreateFile)
        .expect("failed to create default template");
    Ok(())
}
