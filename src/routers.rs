use std::collections::HashMap;

use serde::Serialize;
use warp::{hyper::StatusCode, reject::Reject};
use uuid;
use crate::{
    db::Store,
    models::{Answer, AnswerId, Question, QuestionId},
};

struct Pagination {
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub enum RouterError {
    ParseError(std::num::ParseIntError),
    MissingError,
    QuestionNotFound,
    ContentNotGiven,
}

impl Reject for RouterError {}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            RouterError::MissingError => write!(f, "Only one of them is sent!"),
            RouterError::ParseError(ref err) => write!(f, "Parse Error!{err}"),
            RouterError::QuestionNotFound => write!(f, "Question Not found!"),
            RouterError::ContentNotGiven => write!(f, "Content Not Given!"),
        }
    }
}

fn extract_pagination(params: HashMap<String, i32>) -> Result<Pagination, RouterError> {
    if params.contains_key("start") && params.contains_key("end") {
        Ok(Pagination {
            start: *params.get("start").unwrap() as usize,
            end: *params.get("end").unwrap() as usize,
        })
    } else {
        Err(RouterError::MissingError)
    }
}

pub(crate) async fn get_questions(
    store: Store,
    params: HashMap<String, i32>,
) -> Result<impl warp::Reply, warp::Rejection> {
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

#[derive(Serialize, Hash, Eq, PartialEq)]
enum QuestionReturn<'a> {
    Question(&'a Question),
    Answers(Vec<Answer>),
}

pub(crate) async fn get_question(
    id: QuestionId,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {

    let read_write_rock_on_questions = store.questions.read().await;

    let question = match read_write_rock_on_questions.get(&id) {
        Some(q) => q,
        None => return Err(warp::reject::custom(RouterError::QuestionNotFound)),
    };

    let answers = store
        .answers
        .read()
        .await
        .values()
        .filter(|answer| answer.question_id == id)
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
        None => return Err(warp::reject::custom(RouterError::QuestionNotFound)),
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
        None => Err(warp::reject::custom(RouterError::QuestionNotFound)),
    }
}

pub(crate) async fn add_answer(
    question_id: QuestionId,
    store: Store,
    body: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !body.contains_key("content") {
        return Err(warp::reject::custom(RouterError::ContentNotGiven));
    }
    let content = body.get("content").unwrap();

    if !store.questions.read().await.contains_key(&question_id) {
        return Err(warp::reject::custom(RouterError::QuestionNotFound));
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
