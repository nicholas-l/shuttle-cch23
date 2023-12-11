use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

/// 🎄 Day 4: What do you call a serialized reindeer? Serdeer!

/// Under the soft glow of the Northern Lights, Santa's reindeer are training for the big night. But, oh deer! The reindeer's stats have been serialized into an unknown format after a playful elf accidentally spilled hot cocoa on the central computer. The data needs to be deserialized so the reindeer team can be assembled based on their combined strength.
/// ⭐ Task 1: Reindeer cheer

/// The task is to create a POST endpoint /4/strength that calculates the combined strength of a group of reindeer, so that Santa knows if they can pull his sled through the skies.

/// The input to the endpoint is a JSON array containing information about each reindeer. Each reindeer is represented as an object with two attributes: "name" (string) and "strength" (integer). Collect the strength of each reindeer and respond with the sum.
/// 🔔 Tips

///     serde
///     serde_json
///     JSON in Axum
///     JSON in Actix Web
///     JSON in Rocket

/// 💠 Example

/// curl -X POST http://localhost:8000/4/strength \
///   -H 'Content-Type: application/json' \
///   -d '[
///     { "name": "Dasher", "strength": 5 },
///     { "name": "Dancer", "strength": 6 },
///     { "name": "Prancer", "strength": 4 },
///     { "name": "Vixen", "strength": 7 }
///   ]'

/// 22

pub(crate) fn routes() -> Router {
    Router::new()
        .route("/4/strength", post(part1))
        .route("/4/contest", post(part2))
}

#[derive(Deserialize, Debug)]
struct Reindeer {
    // name: String,
    strength: isize,
}

async fn part1(Json(reindeers): Json<Vec<Reindeer>>) -> String {
    reindeers
        .into_iter()
        .map(|reindeer| reindeer.strength)
        .sum::<isize>()
        .to_string()
}

/// 🎁 Task 2: Cursed candy eating contest (150 bonus points)
/// This time, there is some more data for each reindeer (see example). The endpoint is called /4/contest, because the reindeer are now going to compare the attributes of the reindeer and present a summary of the winners.
/// There is at least one reindeer participating in the contest, and there is never a tie for first place.

/// For some reason, one of the field names in the input seems to still be a bit corrupted from the incident. Guess we just have to deal with it anyways.

/// The output should be a JSON object containing a summary of the winners (see example).
/// 💠 Example Input

/// curl -X POST http://localhost:8000/4/contest \
///   -H 'Content-Type: application/json' \
///   -d '[
///     {
///       "name": "Dasher",
///       "strength": 5,
///       "speed": 50.4,
///       "height": 80,
///       "antler_width": 36,
///       "snow_magic_power": 9001,
///       "favorite_food": "hay",
///       "cAnD13s_3ATeN-yesT3rdAy": 2
///     },
///     {
///       "name": "Dancer",
///       "strength": 6,
///       "speed": 48.2,
///       "height": 65,
///       "antler_width": 37,
///       "snow_magic_power": 4004,
///       "favorite_food": "grass",
///       "cAnD13s_3ATeN-yesT3rdAy": 5
///     }
///   ]'

/// 💠 Example Output

/// Note: JSON output examples are sometimes formatted. Output from your endpoint does not need to be formatted. The output is parsed and checked as a value.

/// {
///   "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
///   "tallest": "Dasher is standing tall with his 36 cm wide antlers",
///   "magician": "Dasher could blast you away with a snow magic power of 9001",
///   "consumer": "Dancer ate lots of candies, but also some grass"
/// }
///
#[derive(Deserialize, Debug)]
struct ReindeerP2 {
    name: String,
    strength: isize,
    speed: f64,
    height: isize,
    antler_width: isize,
    snow_magic_power: isize,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: usize,
}

#[derive(Serialize)]
struct Part2Response {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn part2(Json(reindeers): Json<Vec<ReindeerP2>>) -> impl IntoResponse {
    let fastest = reindeers
        .iter()
        .max_by(|a, b| a.speed.partial_cmp(&b.speed).unwrap())
        .unwrap();

    let tallest = reindeers
        .iter()
        .max_by(|a, b| a.height.cmp(&b.height))
        .unwrap();

    let magician = reindeers
        .iter()
        .max_by(|a, b| a.snow_magic_power.cmp(&b.snow_magic_power))
        .unwrap();

    let consumer = reindeers
        .iter()
        .max_by(|a, b| a.candies_eaten_yesterday.cmp(&b.candies_eaten_yesterday))
        .unwrap();

    Json(Part2Response {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!(
            "{} ate lots of candies, but also some {}",
            consumer.name, consumer.favorite_food
        ),
    })
}

#[cfg(test)]
mod test_day04 {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn part2_test() -> anyhow::Result<()> {
        let app = routes();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/4/contest")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&json!([
                          {
                            "name": "Dasher",
                            "strength": 5,
                            "speed": 50.4,
                            "height": 80,
                            "antler_width": 36,
                            "snow_magic_power": 9001,
                            "favorite_food": "hay",
                            "cAnD13s_3ATeN-yesT3rdAy": 2
                          },
                          {
                            "name": "Dancer",
                            "strength": 6,
                            "speed": 48.2,
                            "height": 65,
                            "antler_width": 37,
                            "snow_magic_power": 4004,
                            "favorite_food": "grass",
                            "cAnD13s_3ATeN-yesT3rdAy": 5
                          }
                        ]))
                        .unwrap(),
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
              "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
               "tallest": "Dasher is standing tall with his 36 cm wide antlers",
               "magician": "Dasher could blast you away with a snow magic power of 9001",
               "consumer": "Dancer ate lots of candies, but also some grass"
            })
        );

        Ok(())
    }
}
