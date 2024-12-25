HTTP requests and responses for [`harpiya`].

## Feature flags

The following optional features are available:

| Name                 | Description                                            | Default? |
|----------------------|--------------------------------------------------------|----------|
| `auth`               | Enables the authentication and authorization.          | No       |
| `cookie`             | Enables the support for cookies.                       | No       |
| `debug`              | Enables the features for ease of debugging.            | No       |
| `i18n`               | Enables the support for internationalization.          | No       |
| `jwt`                | Enables the support for JSON Web Token.                | No       |
| `metrics`            | Enables the [`metrics`] exporter.                      | No       |
| `view`               | Enables the HTML template rendering.                   | No       |

[`metrics`]: https://crates.io/crates/metrics
