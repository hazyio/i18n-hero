pub mod rust_ii8n;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, clap::ValueEnum, Clone, PartialEq, Eq, Hash)]
pub enum LoadersKind {
    RustI18n,
}

impl LoadersKind {
    // pub fn load(&self) -> &'static str {
    //     match self {
    //         LoadersKind::RustI18n => "Rust i18n",
    //     }
    // }
}
