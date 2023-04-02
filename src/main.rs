use std::{str::FromStr};
use std::io::{Error,ErrorKind};
use warp::Filter;
use serde::Serialize;
use warp::cors::CorsForbidden;
use warp::reject::Reject;


#[derive(Default,Debug,Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Default,Debug,Serialize)]
struct QuestionId(String);

impl FromStr for QuestionId{
    type Err = std::io::Error;
    fn from_str(id:&str)->Result<Self,Self::Err>{
        match id.is_empty(){
            false => Ok(Self(id.to_string())),
            true => Err(
                Error::new(ErrorKind::InvalidInput,"No id provided")
            )
        }
    }
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }    
}

//custom error type
#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId{}

async fn get_questions()-> Result<impl warp::Reply,warp::Rejection>{
    let question = Question::new(
        QuestionId::from_str("1").unwrap(),
        "First Question".to_string(),
        "Random Content".to_string(),
        Some(vec!["faq".to_string()])
    );
    match question.id.0.parse::<i32>(){
        Err(_) => {
            Err(warp::reject::custom(InvalidId))
        },
        Ok(_) => {
            Ok(warp::reply::json(&question))
        }
    }
}

//error handler
async fn return_error(r:warp::Rejection) -> Result<impl warp::Reply, warp::Rejection>{
    
    if let Some(error) = r.find::<CorsForbidden>(){
        Ok(warp::reply::with_status(error.to_string(), warp::http::StatusCode::FORBIDDEN))
    }
    else if let Some(InvalidId) = r.find::<InvalidId>(){
        Ok(warp::reply::with_status("Test Rejection".to_string(), warp::http::StatusCode::UNPROCESSABLE_ENTITY))
    } else{
        Ok(warp::reply::with_status(
            "No Such Route!".to_string(), warp::http::StatusCode::NOT_FOUND))
    }
}

#[tokio::main]
async fn main() {
    println!("Server starts...");
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type") 
        .allow_methods(
            &[warp::http::Method::PUT,warp::http::Method::DELETE,warp::http::Method::GET,warp::http::Method::POST]
        );

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions) //Result<impl warp::Reply, warp::Rejection>
        .recover(return_error);

    let routes = get_items.with(cors);

    warp::serve(routes)
        .run(([127,0,0,1],3030))
        .await
}

