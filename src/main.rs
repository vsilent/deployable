mod chat;

use std::collections::HashMap;
use futures::{StreamExt, TryStreamExt};
use std::convert::Infallible;
use warp::{http::StatusCode, http::HeaderMap, multipart::{FormData, Part}, Filter, Rejection, Reply, Stream};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Question {
    question: String
}


#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin",
                            "Access-Control-Request-Method", "Access-Control-Request-Headers",
                            "Content-Type", "Accept", "Authorization", "content-type", "type"])
        .allow_methods(vec!["GET", "POST", "OPTIONS"]);

    let ask_route = warp::filters::body::content_length_limit(1024 * 16)
        .and(log_headers())
        .and(warp::filters::body::json::<Question>())
        .and_then(ask)
        .with(&cors);

    let router = ask_route
        .recover(handle_rejection);

    let port = 8090;
    println!("Server started at localhost: {}", &port);

    warp::serve(router).run(([0, 0, 0, 0], port)).await;
}

fn log_headers() -> impl Filter<Extract=(), Error=Infallible> + Copy {
    warp::header::headers_cloned()
        .map(|headers: HeaderMap| {
            for (k, v) in headers.iter() {
                println!("{}: {}", k, v.to_str().expect("Failed to print header value"))
            }
        })
        .untuple_one()
}

async fn ask(post_data: Question) -> Result<impl Reply, Rejection> {
    // let response = format!("You asked {:?}", post_data.question);
    // Creating a new conversation
    let response = chat::ask(post_data.question).await;
    Ok(warp::reply::json(&response.unwrap()))
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}
