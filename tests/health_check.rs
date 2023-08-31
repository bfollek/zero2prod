//! tests/health_check.rs
//!
// `tokio::test` is the testing equivalent of `tokio::main`.
// It also spares you from having to specify the `#[test]` attribute.
//
// You can inspect what code gets generated using
// `cargo expand --test health_check` (<- name of the test file)

#[tokio::test]
async fn health_check_works() {
    //  Arrange
    // No .await, no .expect
    spawn_app();
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// No .await call, therefore no need for `spawn_app` to be async now.
// We are also running tests, so it is not worth it to propagate errors:
// if we fail to perform the required setup we can just panic and crash
// all the things.
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let

    // "A second look at tokio::spawn’s documentation supports our hypothesis:
    // when a tokio runtime is shut down all tasks spawned on it are dropped.
    // tokio::test spins up a new runtime at the beginning of each test case and
    // they shut down at the end of each test case. In other words, good news -
    // no need to implement any clean up logic to avoid leaking resources between test
    // runs." - p. 36
    let _ = tokio::spawn(server);
}
