use std::fmt::Display;

use super::{Linux, Unix, Windows};

#[derive(Debug)]

/// this is used as a unified handler to be stored in the OllamaDownload struct as a single field !!!
pub enum Platform {
    Linux(Linux),
    Windows(Windows),
    Unix(Unix),
}
impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = match self {
            Self::Linux(l) => l.to_string(),
            Self::Unix(u) => u.to_string(),
            Self::Windows(w) => w.to_string(),
        };
        write!(f, "{}", p)
    }
}
