// api_routes.rs
use formvault::test_helpers::spawn_app;
use reqwest;
use tokio;

#[tokio::test]
async fn test_get_api_routes() {
    // Start the app
    let addr = spawn_app().await;

    // Build URL
    let url = format!("http://{}/", addr);

    // Send GET request
    let response = reqwest::get(&url)
        .await
        .expect("Failed to connect to server");

    // Check that response is 200 OK
    assert!(response.status().is_success(), "Response was not successful");

    // Optionally, check the JSON structure
    let body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response as JSON");

    println!("Response body: {:?}", body);

    // Example assertion: body is an array
    assert!(body.is_array(), "Expected JSON array for API map");
}
