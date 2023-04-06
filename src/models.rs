use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct QuestionId(pub String);

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(Self(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

impl Answer {
    pub fn new(id: &str, content: &str, question_id: QuestionId) -> Self {
        Self {
            id: AnswerId(id.to_string()),
            content: content.to_string(),
            question_id: question_id,
        }
    }
}
#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct AnswerId(pub String);
