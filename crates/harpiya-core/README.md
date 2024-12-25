Core types and traits for [`harpiya`].

## Feature flags

The following optional features are available:

| Name                 | Description                                            | Default? |
|----------------------|--------------------------------------------------------|----------|
| `apalis`             | Enables the support for [`apalis`].                    | No       |
| `cookie`             | Enables the support for cookies.                       | No       |
| `crypto-sm`          | Enables China's Standards of Encryption Algorithms.    | No       |
| `debug`              | Enables the features for ease of debugging.            | No       |
| `dotenv`             | Enables the configuration loader for a `.env` file.    | No       |
| `env-filter`         | Enables the `env-filter` for [`tracing-subscriber`].   | No       |
| `http-client`        | Enables the HTTP client via [`reqwest`].               | No       |
| `locale`             | Enables the support for locale related utilities.      | No       |
| `metrics`            | Enables the [`metrics`] exporter.                      | No       |
| `runtime-async-std`  | Enables the [`async-std`] runtime.                     | No       |
| `runtime-tokio`      | Enables the [`tokio`] runtime.                         | No       |
| `sentry`             | Enables the integration with [`sentry`].               | No       |
| `tls-native`         | Enables the [`native-tls`] TLS backend.                | No       |
| `tls-rustls`         | Enables the [`rustls`] TLS backend.                    | No       |
| `tracing-log`        | Enables the `tracing-log` for [`tracing-subscriber`].  | No       |
| `tracing-subscriber` | Enables the integration with [`tracing-subscriber`].   | No       |
| `validator`          | Enables the common validation rules.                   | No       |

[`apalis`]: https://crates.io/crates/apalis
[`tracing-subscriber`]: https://crates.io/crates/tracing-subscriber
[`reqwest`]: https://crates.io/crates/reqwest
[`metrics`]: https://crates.io/crates/metrics
[`async-std`]: https://crates.io/crates/async-std
[`tokio`]: https://crates.io/crates/tokio
[`native-tls`]: https://crates.io/crates/native-tls
[`rustls`]: https://crates.io/crates/rustls
[`sentry`]: https://crates.io/crates/sentry
