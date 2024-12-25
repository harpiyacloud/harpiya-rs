#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![allow(async_fn_in_trait)]
#![forbid(unsafe_code)]

mod helper;

pub mod request;
pub mod response;
pub mod timing;

#[cfg(feature = "i18n")]
pub mod i18n;

#[cfg(feature = "view")]
pub mod view;

#[cfg(feature = "i18n")]
#[doc(no_inline)]
pub use fluent::fluent_args;
