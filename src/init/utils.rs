use std::fs;

use crate::init::error::InitError;

const DEFAULT_STYLES: &'static str = include_str!("../../static/styles.css");
const DEFAULT_TEMPLATE: &'static str = include_str!("../../templates/post.html");

pub fn init() -> Result<(), InitError> {
    fs::create_dir_all("static")
        .map_err(|_| InitError::FailedToCreateDir)
        .expect("failed to create static dir");
    fs::create_dir_all("templates")
        .map_err(|_| InitError::FailedToCreateDir)
        .expect("failed to create templates dir");
    fs::write("static/styles.css", DEFAULT_STYLES).expect("failed to create default styles");
    fs::write("templates/post.html", DEFAULT_TEMPLATE).expect("failed to create default template");
    Ok(())
}
