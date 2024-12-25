#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]
#![allow(async_fn_in_trait)]
#![forbid(unsafe_code)]

mod application;
mod controller;
mod middleware;
mod request;
mod response;

pub mod prelude;

pub use application::Cluster;
pub use request::Extractor;
pub use response::{AxumRejection, AxumResponse};

/// A specialized request extractor.
pub type Request = Extractor<axum::http::Request<axum::body::Body>>;

/// A specialized response.
pub type Response = harpiya_http::response::Response<axum::http::StatusCode>;

/// A specialized `Result` type.
pub type Result<T = AxumResponse> = std::result::Result<T, AxumRejection>;

pub use controller::DefaultController;
