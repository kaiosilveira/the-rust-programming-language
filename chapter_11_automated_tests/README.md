# Automated tests

| Command                          | Description                                                            |
| -------------------------------- | ---------------------------------------------------------------------- |
| `cargo test`                     | runs all the tests                                                     |
| `cargo test -- --test-threads=1` | runs all the tests in a single thread (sequentially)                   |
| `cargo test -- --show-output`    | runs all the tests with outputs                                        |
| `cargo test <function_name>`     | runs a test function that matches the name passed in `<function_name>` |
| `cargo test str_pattern`         | runs all tests that matches the value of `str_pattern`                 |
| `cargo test -- --ignored         | runs all the tests marked with the `[ignored]` annotation              |

## Other notes

- Rust allows to execute private functions inside tests
- Unit tests are generally kept at the same file as the code being tested
- integration tests are usually kept inside a separate `tests` directory, at the same level as `src`
