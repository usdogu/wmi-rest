pub mod memory;
pub mod network;
pub mod processor;
pub mod vhd;
pub mod vm;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Result = std::result::Result<String, Error>;
