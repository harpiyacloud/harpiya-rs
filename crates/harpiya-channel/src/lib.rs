#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod cloud_event;
mod subscription;

pub use cloud_event::CloudEvent;
pub use subscription::Subscription;

#[cfg(feature = "flume")]
mod flume;

#[cfg(feature = "flume")]
pub use flume::MessageChannel;
