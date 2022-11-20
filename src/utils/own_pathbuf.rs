use rocket::http::uri::{error::PathError, fmt::Path, Segments};
use rocket::request::FromSegments;
use std::path::PathBuf;

pub struct PathBufWithDotfiles(PathBuf);

impl FromSegments<'_> for PathBufWithDotfiles {
    type Error = PathError;

    fn from_segments(segments: Segments<'_, Path>) -> Result<Self, Self::Error> {
        segments.to_path_buf(true).map(Self)
    }
}

impl PathBufWithDotfiles {
    pub fn get(&self) -> PathBuf {
        self.0.clone()
    }
}
