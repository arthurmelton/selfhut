use std::path::PathBuf;
use rocket::http::uri::{Segments, error::PathError, fmt::Path};
use rocket::request::FromSegments;

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
