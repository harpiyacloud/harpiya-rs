use serde::{Deserialize, Serialize};
use harpiya_rs::prelude::*;
use harpiya_derive::{DecodeRow, Entity, Model, ModelAccessor, ModelHooks, Schema};

/// The `tenant` model.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    DecodeRow,
    Schema,
    ModelAccessor,
    ModelHooks,
    Model,
    Entity,
)]
#[serde(default)]
#[schema(auto_rename)]
pub struct Tenant {
    // Basic fields.
    #[schema(primary_key, read_only, constructor = "Uuid::now_v7")]
    id: Uuid,
    #[schema(not_null, comment = "Tenant name")]
    name: String,
    #[schema(default_value = "Active", index_type = "hash")]
    status: String,
    description: String,

    // Extensions.
    #[schema(reserved)]
    extra: Map,

    // Revisions.
    #[schema(read_only, default_value = "now", index_type = "btree")]
    created_at: DateTime,
    #[schema(default_value = "now", index_type = "btree")]
    updated_at: DateTime,
    version: u64,
}