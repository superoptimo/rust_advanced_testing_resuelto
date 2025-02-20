//! Write a custom matcher that matches if:
//!
//! - The method is `POST`
//! - The `Content-Type` header is present and set to `application/json`
//! - The request body is a valid JSON object
//! - The `Content-Length` header is set and its value matches the length of the request body (in bytes)
use core::panic;

use wiremock::{Match, Request};

struct WellFormedJson;

impl Match for WellFormedJson {
    fn matches(&self, request: &Request) -> bool {
        // - The method is `POST`
        if request.method.ne(&wiremock::http::Method::POST) {
            return false;
        }

        // - The `Content-Type` header is present and set to `application/json`
        let has_header_content:bool = match request.headers.get("content-type") {
            Some(val) => {
                // - The `Content-Type` header is present and set to `application/json`
                val.to_str().map_or_else(|err| {
                    panic!("Error parse! {0}", err);
                    false
                },
                |strval| strval.eq("application/json"))
            },
            None => {
                //panic!("No tiene Header");
                false
            }
        };

        if !has_header_content {
            // panic!("Fallo Header");
            return false;
        }

        // - The `Content-Length` header is set and its value matches the length of the request body (in bytes)
        let has_header_contentlen = match request.headers.get("content-length") {
            Some(val) => {
                val.to_str().map_or_else(|err| {
                    panic!("Error! {0}", err);
                    false
                },
                 |strval|{
                    let lval = strval.parse::<usize>().expect("Cannot calc length uszie");
                    lval == request.body.len()
                })
            },
            None => {
                panic!("No tiene len");
                false
            }
        };

        if has_header_contentlen == false {
            panic!("Len not equal Error");
            return false;
        }
        
        // - The request body is a valid JSON object
        let body_str = std::str::from_utf8(request.body.as_slice());

        if let Ok(valid_str) = body_str {
            let res = serde_json::from_str::<serde_json::Value>(valid_str).map_or_else(|err|{
                // panic!("Error Serde Json : {0}", err);
                false
            },
            |_| true);

            res
        }
        else {
            panic!("Not valid UTF8");
            false            
        }
        
        
    }
}

#[cfg(test)]
mod tests {
    use crate::WellFormedJson;
    use googletest::assert_that;
    use googletest::matchers::eq;
    use serde_json::json;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn test_server() -> MockServer {
        let server = MockServer::start().await;
        server
            .register(Mock::given(WellFormedJson).respond_with(ResponseTemplate::new(200)))
            .await;
        server
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_invalid_json() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        // Trailing comma is not valid in JSON
        let body = r#"{"hi": 2,"#;
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .header("Content-Type", "application/json")
            .body(r#"{"hi": 2,"#)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_missing_content_type() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&json!({"hi": 2})).unwrap();
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .body(body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_invalid_content_length() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = serde_json::to_string(&json!({"hi": 2})).unwrap();
        let length = body.len();

        let outcome = client
            .post(&server.uri())
            .header("Content-Length", length)
            .body(body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn errors_on_non_post() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = json!({"hi": 2});

        let outcome = client
            .patch(&server.uri())
            .json(&body)
            .send()
            .await
            .unwrap();
        assert_that!(outcome.status().as_u16(), eq(404));
    }

    #[googletest::gtest]
    #[tokio::test]
    async fn happy_path() {
        let server = test_server().await;
        let client = reqwest::Client::new();
        let body = json!({"hi": 2});

        let outcome = client.post(&server.uri()).json(&body).send().await.unwrap();
        assert_that!(outcome.status().as_u16(), eq(200));
    }
}
