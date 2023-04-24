use std::{process::Termination};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// An error received from the API.
#[derive(Debug, Default)]
pub struct MystbinError {
    pub code: u16,
}

/// The base file.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct File {
    pub filename: String,
    pub content: String,
}

/// The base paste.
#[derive(Debug, Default)]
pub struct Paste {
    pub created_at: DateTime<FixedOffset>,
    pub expires: Option<DateTime<FixedOffset>>,
    pub files: Vec<File>,
    pub id: String,
}

impl Termination for Paste {
    fn report(self) -> std::process::ExitCode {
        todo!()
    }
}

/// The result obtained from delete_paste and delete_paste functions.
#[derive(Debug, Default)]
pub struct DeleteResult {
    pub succeeded: Option<Vec<String>>,
    pub failed: Option<Vec<String>>,
}

/// The base user paste. This does not contain the files from the paste.
#[derive(Debug)]
pub struct UserPaste {
    pub created_at: DateTime<FixedOffset>,
    pub expires: Option<DateTime<FixedOffset>>,
    pub id: String,
}

impl Termination for UserPaste {
    fn report(self) -> std::process::ExitCode {
        todo!()
    }
}