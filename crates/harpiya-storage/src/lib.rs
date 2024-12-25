#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod file;

#[cfg(feature = "accessor")]
mod accessor;

pub use file::NamedFile;

#[cfg(feature = "accessor")]
pub use accessor::GlobalAccessor;
