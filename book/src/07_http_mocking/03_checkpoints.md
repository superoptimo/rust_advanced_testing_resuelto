# Checkpoints

When a `MockServer` instance goes out of scope (i.e. when it's dropped), it will verify that all the expectations
that have been set on its registered mocks have been satisfied.

When you have a complex mocking setup, it can be useful to verify the state of the mocks _before_ the end
of the test.\
`wiremock` provides two methods for this purpose:

- [`MockServer::verify`](https://docs.rs/wiremock/latest/wiremock/struct.MockServer.html#method.verify) verifies
  that all the expectations have been satisfied. It panics if they haven't.
- Scoped mocks,
  via [`MockServer::register_as_scoped`](https://docs.rs/wiremock/latest/wiremock/struct.MockServer.html#method.register_as_scoped).

`verify` is self-explanatory, so let's dive into scoped mocks.

## Scoped mocks

When you register a mock with `MockServer::register`, it will be active until the `MockServer` instance goes out of
scope.\
`MockServer::register_scoped`, instead, returns
a [`MockGuard`](https://docs.rs/wiremock/latest/wiremock/struct.MockGuard.html).\
The mock will be active until the guard is alive. When the guard goes out of scope, the mock will be removed from the
`MockServer` instance and its expectations will be verified.
