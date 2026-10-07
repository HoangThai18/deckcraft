//! PPTX import and export (in progress).
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#![forbid(unsafe_code)]

use slidecraft_model::Presentation;

#[derive(Debug, thiserror::Error)]
pub enum PptxError {
    #[error("not a PPTX file: {0}")]
    NotPptx(String),
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error("write failed: {0}")]
    Write(String),
}

pub fn sniff(bytes: &[u8]) -> bool {
    bytes.starts_with(b"PK") && bytes.windows(19).take(4096).any(|w| w == b"[Content_Types].xml")
}

pub fn import(_bytes: &[u8]) -> Result<Presentation, PptxError> {
    Err(PptxError::Unsupported("PPTX import is being implemented".into()))
}

pub fn export(_p: &Presentation) -> Result<Vec<u8>, PptxError> {
    Err(PptxError::Unsupported("PPTX export is being implemented".into()))
}
