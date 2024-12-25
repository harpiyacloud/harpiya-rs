use super::Schema;
use std::fmt::Display;
use harpiya_core::{
    crypto,
    encoding::base64,
    error::Error,
    extension::{JsonObjectExt, TomlTableExt},
    state::State,
    warn, LazyLock, Map,
};

/// Helper utilities for models.
pub trait ModelHelper<K>: Schema<PrimaryKey = K>
where
    K: Default + Display + PartialEq,
{
    /// Returns the secret key for the model.
    /// It should have at least 64 bytes.
    ///
    /// # Note
    ///
    /// This should only be used for internal services. Do not expose it to external users.
    #[inline]
    fn secret_key() -> &'static [u8] {
        SECRET_KEY.as_slice()
    }

    /// Encrypts the password for the model.
    fn encrypt_password(password: &str) -> Result<String, Error> {
        let key = Self::secret_key();
        let password = password.as_bytes();
        if base64::decode(password).is_ok_and(|bytes| bytes.len() == 256) {
            crypto::encrypt_hashed_password(password, key)
                .map_err(|err| warn!("fail to encrypt hashed password: {}", err.message()))
        } else {
            crypto::encrypt_raw_password(password, key)
                .map_err(|err| warn!("fail to encrypt raw password: {}", err.message()))
        }
    }

    /// Verifies the password for the model.
    fn verify_password(password: &str, encrypted_password: &str) -> Result<bool, Error> {
        let key = Self::secret_key();
        let password = password.as_bytes();
        let encrypted_password = encrypted_password.as_bytes();
        if base64::decode(password).is_ok_and(|bytes| bytes.len() == 256) {
            crypto::verify_hashed_password(password, encrypted_password, key)
                .map_err(|err| warn!("fail to verify hashed password: {}", err.message()))
        } else {
            crypto::verify_raw_password(password, encrypted_password, key)
                .map_err(|err| warn!("fail to verify raw password: {}", err.message()))
        }
    }

    /// Translates the model data.
    fn translate_model(model: &mut Map) {
        #[cfg(feature = "openapi")]
        harpiya_openapi::translate_model_entry(model, Self::model_name());
        for col in Self::columns() {
            if let Some(translated_field) = col.extra().get_str("translate_as") {
                let field = [col.name(), "_translated"].concat();
                if let Some(value) = model.remove(&field) {
                    model.upsert(translated_field, value);
                }
            }
        }
    }
}

impl<M, K> ModelHelper<K> for M
where
    M: Schema<PrimaryKey = K>,
    K: Default + Display + PartialEq,
{
}

/// Secret key.
static SECRET_KEY: LazyLock<[u8; 64]> = LazyLock::new(|| {
    let app_config = State::shared().config();
    let config = app_config.get_table("database").unwrap_or(app_config);
    let checksum: [u8; 32] = config
        .get_str("checksum")
        .and_then(|checksum| checksum.as_bytes().try_into().ok())
        .unwrap_or_else(|| {
            let secret = config
                .get_str("secret")
                .map(|s| s.to_owned())
                .unwrap_or_else(|| {
                    tracing::warn!("auto-generated `secret` is used for deriving a secret key");
                    format!("{}{}", *super::TABLE_PREFIX, super::DRIVER_NAME)
                });
            crypto::digest(secret.as_bytes())
        });
    let info = config.get_str("info").unwrap_or("HARPIYA:ORM");
    crypto::derive_key(info, &checksum)
});
