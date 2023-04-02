use std::collections::HashMap;

use warp::reject::Reject;

use crate::{db, models::Question};

//custom error type
#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

#[derive(Debug)]
pub enum RouterError {
    ParseError(std::num::ParseIntError),
    MissingParameters,
}

//to format error,
impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RouterError::ParseError(ref err) => {
                write!(f, "Cannot parse parameter {}", err)
            }
            RouterError::MissingParameters => write!(f, "Missing parameter"),
        }
    }
}

impl Reject for RouterError {}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, RouterError> {
    if params.contains_key("start") && params.contains_key("end") {
        return Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(RouterError::ParseError)?,
            end: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(RouterError::ParseError)?,
        });
    };
    Err(RouterError::MissingParameters)
}

pub(crate) async fn get_questions(
    params: HashMap<String, String>,
    store: db::Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    dbg!(&params);
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub(crate) async fn add_question(
    store: db::Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status(
        "Question Added",
        warp::http::StatusCode::OK,
    ))
}
