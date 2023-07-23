pub mod memory;
pub mod vm;
pub mod processor;
pub mod network;
pub mod vhd;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Result = std::result::Result<String, Error>;