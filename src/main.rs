mod db;
pub mod models;
pub mod routers;
use routers::RouterError;
use warp::cors::CorsForbidden;
use warp::filters::body::BodyDeserializeError;
use warp::hyper::StatusCode;
use warp::Filter;
use crate::models::QuestionId;

//error handler
async fn return_error(r: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<RouterError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "No Such Route!".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    println!("Server starts...");
    let store = db::Store::new();
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
        .and_then(routers::get_questions); //Result<impl warp::Reply, warp::Rejection>

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<QuestionId>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routers::get_question);

    let add_questions = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routers::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routers::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routers::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::param::<QuestionId>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routers::add_answer);

    let routes = get_questions
        .or(get_question)
        .or(add_questions)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await
}
