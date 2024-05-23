use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, Debug)]
pub enum OBSCargoErrorKind {
    AuditNeedsAction,
    VendorCompressionFailed,
    VendorError,
    AuditError,
    LockFileError,
}

impl OBSCargoErrorKind {
    pub(crate) fn as_str(self) -> &'static str {
        use OBSCargoErrorKind::*;
        match self {
            AuditError => "cargo audit process failed",
            AuditNeedsAction => "security audit is actionable",
            VendorError => "cargo vendor process failed",
            VendorCompressionFailed => "compress vendored dependencies failed",
            LockFileError => "lockfile generation failed",
        }
    }
}

#[derive(Clone)]
pub struct OBSCargoError {
    kind: OBSCargoErrorKind,
    message: String,
}

impl Error for OBSCargoError {}

impl Debug for OBSCargoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_msg = format!("kind: {}\nreason: {}", self.kind.as_str(), self.message);
        write!(f, "{}", full_msg)
    }
}

impl Display for OBSCargoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl OBSCargoError {
    pub(crate) fn new(kind: OBSCargoErrorKind, message: String) -> OBSCargoError {
        Self { kind, message }
    }
}
