use crate::types::pagination::extract_pagination;
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
use warp::hyper::StatusCode;

pub(crate) async fn get_questions(
    store: Store,
    params: HashMap<String, i32>,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("Start querying questions");
    if !params.is_empty() {
        let pagination = extract_pagination(params).map_err(|_| Error::NotParsable)?;
        log::info!("Pagination set {:?}", &pagination);

        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        log::info!("No pagination used");
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub(crate) async fn get_question(
    id: QuestionId,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let read_write_rock_on_questions = store.questions.read().await;
    let question = match read_write_rock_on_questions.get(&id) {
        Some(q) => q,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    };
    let answers = store
        .answers
        .read()
        .await
        .values()
        .filter(|a| a.question_id == id)
        .cloned()
        .collect::<Vec<Answer>>();
    let mut res = HashMap::new();
    res.insert(id.0.to_string(), QuestionReturn::Question(question));
    res.insert("answers".to_string(), QuestionReturn::Answers(answers));
    Ok(warp::reply::json(&res))
}

pub(crate) async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status(
        "Question Added1",
        warp::http::StatusCode::OK,
    ))
}

pub(crate) async fn update_question(
    id: String,
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
    id: String,
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
