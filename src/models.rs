use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub(crate) struct Question {
    pub id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub(crate) struct QuestionId(String);

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
    pub(crate) fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(Clone)]
pub struct Store {
    questions: HashMap<QuestionId, Question>,
}
