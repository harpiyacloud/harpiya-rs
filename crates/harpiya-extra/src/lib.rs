#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "format")]
pub mod format;
