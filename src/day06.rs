// 🎄 Day 6: Elf on a shelf

// It's that time of year when the elves hide on shelves to watch over the children of the world, reporting back to Santa on who's been naughty or nice. However, this year's reports have been mixed up with the rest of the letters to Santa, and the word "elf" is hidden throughout a mountain of text.
// ⭐ Task 1: Never count on an elf

// Elves are notorious for their hide-and-seek skills, and now they've hidden themselves in strings of text!

// Create an endpoint /6 that takes a POST request with a raw string as input and count how many times the substring "elf" appears.

// The output should be a JSON object containing the count of the string "elf".
// 🔔 Tips

//     Rust Primitive Type str
//     Rust String struct

// 💠 Examples

// curl -X POST http://localhost:8000/6 \
//   -H 'Content-Type: text/plain' \
//   -d 'The mischievous elf peeked out from behind the toy workshop,
//       and another elf joined in the festive dance.
//       Look, there is also an elf on that shelf!'

// {"elf":4}

// 🎁 Task 2: Shelf under an elf? (200 bonus points)

// Add two fields to the response that counts:

//     The number of occurrences of the string "elf on a shelf" in a field with the same name.
//     The number of shelves that don't have an elf on it. That is, the number of strings "shelf" that are not preceded by the string "elf on a ". Put this count in the field "shelf with no elf on it".

// 💠 Example

// curl -X POST http://localhost:8000/6 \
//   -H 'Content-Type: text/plain' \
//   -d 'there is an elf on a shelf on an elf.
//       there is also another shelf in Belfast.'

// {"elf":5,"elf on a shelf":1,"shelf with no elf on it":1}

use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Serialize;

pub(crate) fn routes() -> Router {
    Router::new().route("/", post(part1))
}

#[derive(Serialize, Debug)]
struct Response {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    elf_no_shelf: usize,
}

async fn part1(text: String) -> impl IntoResponse {
    let elf = text.matches("elf").count();
    let elf_shelf = text.matches("elf on a shelf").count();
    let elf_no_shelf = text.matches("shelf").count() - elf_shelf;

    Json(Response {
        elf,
        elf_shelf,
        elf_no_shelf,
    })
}

#[cfg(test)]
mod test_day04 {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn part1_test() -> anyhow::Result<()> {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .body(Body::from(
                        "The mischievous elf peeked out from behind the toy workshop,
                        and another elf joined in the festive dance.
                        Look, there is also an elf on that shelf!",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body: Value =
            serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()[..])
                .unwrap();

        assert_eq!(body["elf"], 4);

        Ok(())
    }

    #[tokio::test]
    async fn part2_test() -> anyhow::Result<()> {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .body(Body::from(
                        "there is an elf on a shelf on an elf.
                        there is also another shelf in Belfast.",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body: Value =
            serde_json::from_slice(&response.into_body().collect().await.unwrap().to_bytes()[..])
                .unwrap();

        assert_eq!(
            body,
            json!({
                "elf": 5,
                "elf on a shelf": 1,
                "shelf with no elf on it": 1,
            })
        );

        Ok(())
    }
}
