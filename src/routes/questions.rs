use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::NewQuestion;
use crate::{
    store::Store,
    types::{
        answer::Answer,
        question::{Question, QuestionId},
    },
};
use handle_errors::Error;
use serde::Serialize;

use std::{collections::HashMap, hash::Hash};
use tracing::{info, instrument};
use warp::hyper::StatusCode;

#[instrument]
pub(crate) async fn get_questions(
    store: Store,
    params: HashMap<String, i32>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut pagination = Pagination::default();

    if !params.is_empty() {
        pagination = extract_pagination(params)?;
        info!(pagination = true);
    }
    //Fetching
    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };
    // Return response
    Ok(warp::reply::json(&res))
}

pub(crate) async fn get_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut res = HashMap::new();

    let question = match store.get_question(&id).await {
        Ok(question) => question,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    res.insert(id.0.to_string(), QuestionReturn::Question(&question));

    if let Ok(answers) = store.get_answers(id).await {
        res.insert("answers".to_string(), QuestionReturn::Answers(answers));
    }

    Ok(warp::reply::json(&res))
}

pub(crate) async fn add_question(
    store: Store,
    question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::with_status("Question Added", StatusCode::Ok)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub(crate) async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question Updated", StatusCode::OK))
}

pub(crate) async fn delete_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "Question Deleted!",
            StatusCode::OK,
        )),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

#[derive(Serialize, Hash, Eq, PartialEq)]
enum QuestionReturn<'a> {
    Question(&'a Question),
    Answers(Vec<Answer>),
}
