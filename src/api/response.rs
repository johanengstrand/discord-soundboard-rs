/// Creates a JSON response containing some data
/// # Arguments
/// * `data` - A JSON object containing the data
///
/// # Examples
/// ```
/// success!([1, 2, 3, 4])
/// // Gives the following JSON response for the endpoint:
/// // {
/// //      "success": true,
/// //      "data": [1, 2, 3, 4]
/// // }
/// ```
#[macro_export]
macro_rules! success {
    ($input:expr) => {
        Ok(warp::reply::json(&serde_json::json!({
            "success": true,
            "data": $input
        })))
    }
}

/// Creates a JSON error response with a message
/// # Arguments
/// * `message` - A string containing the error message
///
/// # Examples
/// ```
/// failure!("Failed to fetch tracks")
/// // Gives the following JSON response for the endpoint:
/// // {
/// //      "success": false,
/// //      "error": "Failed to fetch tracks"
/// // }
/// ```
#[macro_export]
macro_rules! failure {
    ($input:expr) => {
        Ok(warp::reply::json(&serde_json::json!({
            "success": false,
            "error": $input
        })))
    }
}
