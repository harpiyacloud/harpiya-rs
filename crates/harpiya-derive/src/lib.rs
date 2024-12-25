#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod decode_row;
mod entity;
mod model;
mod model_accessor;
mod model_hooks;
mod parser;
mod schema;

#[doc = include_str!("../docs/entity.md")]
#[proc_macro_derive(Entity, attributes(schema))]
pub fn derive_entity(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let output = entity::parse_token_stream(input);
    TokenStream::from(output)
}

#[doc = include_str!("../docs/schema.md")]
#[proc_macro_derive(Schema, attributes(schema))]
pub fn derive_schema(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let output = schema::parse_token_stream(input);
    TokenStream::from(output)
}

#[doc = include_str!("../docs/model_accessor.md")]
#[proc_macro_derive(ModelAccessor, attributes(schema))]
pub fn derive_model_accessor(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let output = model_accessor::parse_token_stream(input);
    TokenStream::from(output)
}

#[doc = include_str!("../docs/decode_row.md")]
#[proc_macro_derive(DecodeRow, attributes(schema))]
pub fn derive_decode_row(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let output = decode_row::parse_token_stream(input);
    TokenStream::from(output)
}

#[doc = include_str!("../docs/model_hooks.md")]
#[proc_macro_derive(ModelHooks, attributes(schema))]
pub fn derive_model_hooks(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let output = model_hooks::parse_token_stream(input);
    TokenStream::from(output)
}

#[doc = include_str!("../docs/model.md")]
#[proc_macro_derive(Model, attributes(schema))]
pub fn derive_model(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let output = model::parse_token_stream(input);
    TokenStream::from(output)
}
