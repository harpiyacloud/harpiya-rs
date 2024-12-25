//! Re-exports of common types in [`harpiya-core`].

#[doc(no_inline)]
pub use harpiya_core::{
    application::{Application, Plugin},
    bail,
    datetime::{Date, DateTime, Time},
    error::Error,
    extension::{JsonObjectExt, JsonValueExt, TomlTableExt},
    json,
    model::{Model, ModelHooks, Mutation, Query, QueryContext},
    schedule::{AsyncCronJob, AsyncJob, AsyncJobScheduler, CronJob, Job, JobScheduler},
    state::State,
    validation::Validation,
    warn, BoxFuture, Decimal, LazyLock, Map, Record, Uuid,
};

#[doc(no_inline)]
pub use harpiya_storage::NamedFile;

#[cfg(feature = "auth")]
#[doc(no_inline)]
pub use harpiya_auth::{
    AccessKeyId, AuthorizationProvider, SecretAccessKey, SecurityToken, UserSession,
};

#[cfg(feature = "i18n")]
#[doc(no_inline)]
pub use harpiya_http::fluent_args;

#[cfg(feature = "jwt")]
#[doc(no_inline)]
pub use harpiya_auth::JwtClaims;

#[cfg(feature = "opa")]
#[doc(no_inline)]
pub use harpiya_auth::RegoEngine;

#[cfg(feature = "orm")]
#[doc(no_inline)]
pub use harpiya_orm::{
    Aggregation, Entity, IntoSqlValue, JoinOn, ModelAccessor, ModelHelper, MutationBuilder,
    QueryBuilder, ScalarQuery, Schema, Transaction, Window,
};

pub use harpiya_http::{
    reject,
    request::RequestContext,
    response::{ExtractRejection, Rejection, StatusCode, WebHook},
};
