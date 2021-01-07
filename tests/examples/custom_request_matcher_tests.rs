extern crate httpmock;

use isahc::get;

use httpmock::{Mock, MockServer};

#[test]
// TODO: Implement custom matcher
fn my_custom_request_matcher_test() {
    // Arrange
    let _ = env_logger::try_init();

    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.matches(|req| req.path.to_lowercase().ends_with("test"));
        then.status(200);
    });

    // Act: Send the HTTP request
    let response = get(server.url("/thisIsMyTest")).unwrap();

    // Assert
    mock.assert();
    assert_eq!(response.status(), 200);
}