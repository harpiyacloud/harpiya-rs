Files and storage services for [`harpiya`].

## Feature flags

The following optional features are available:

| Name                 | Description                                            | Default? |
|----------------------|--------------------------------------------------------|----------|
| `accessor`           | Enables the data access layer built with [`opendal`].  | No       |
| `http-client`        | Enables the HTTP client via [`reqwest`].               | No       |

[`opendal`]: https://crates.io/crates/opendal
[`reqwest`]: https://crates.io/crates/reqwest
