use warp::cors::CorsForbidden;
use warp::filters::body::BodyDeserializeError;
use warp::hyper::StatusCode;
use warp::reject::Reject;

#[derive(Debug)]
pub enum Error {
    NotParsable,
    Missing,
    QuestionNotFound,
    ContentNotGiven,
}

impl Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Missing => write!(f, "Only one of them is sent!"),
            Error::NotParsable => write!(f, "Parse Error!"),
            Error::QuestionNotFound => write!(f, "Question Not found!"),
            Error::ContentNotGiven => write!(f, "Content Not Given!"),
        }
    }
}

pub async fn return_error(r: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
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
