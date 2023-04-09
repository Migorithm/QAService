use crate::{
    store::Store,
    types::{
        answer::{Answer, AnswerId},
        question::QuestionId,
    },
};
use handle_errors::Error;
use std::collections::HashMap;
use uuid;
use warp::hyper::StatusCode;

pub(crate) async fn add_answer(
    question_id: QuestionId,
    store: Store,
    body: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !body.contains_key("content") {
        return Err(warp::reject::custom(Error::ContentNotGiven));
    }
    let content = body.get("content").unwrap();

    if !store.questions.read().await.contains_key(&question_id) {
        return Err(warp::reject::custom(Error::QuestionNotFound));
    }

    let answer_id = uuid::Uuid::new_v4().to_string();
    store.answers.write().await.insert(
        AnswerId(answer_id.clone()),
        Answer::new(&answer_id.clone(), content, question_id.clone()),
    );
    Ok(warp::reply::with_status(
        format!("Answer Created Against {question_id:?}!"),
        StatusCode::OK,
    ))
}
