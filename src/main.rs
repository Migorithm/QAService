mod routes;
mod store;
mod types;

use crate::types::question::QuestionId;
use handle_errors::return_error;
use warp::Filter;
use tracing_subscriber::fmt::format::FmtSpan; // fmt subscriber, which is meant to format and log events to the console. 
//error handler

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "web-test=info,warp=info".to_owned());

    
    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE) // which indicates that our subscriber will also log the closing of spans.
        .init();


    println!("Server starts...");
    let store = store::Store::default();
    let store_filter = warp::any().map(move || store.clone());
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            warp::http::Method::PUT,
            warp::http::Method::DELETE,
            warp::http::Method::GET,
            warp::http::Method::POST,
        ]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::query())
        .and_then(routes::questions::get_questions); //Result<impl warp::Reply, warp::Rejection>

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<QuestionId>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::questions::get_question);

    let add_questions = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::questions::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::questions::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::questions::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::param::<QuestionId>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(get_question)
        .or(add_questions)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
