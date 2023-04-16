use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct QuestionId(pub i32);

impl FromStr for QuestionId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(Self(id.parse::<i32>().map_err( |_| Error::new(ErrorKind::InvalidInput, "Wrong Id Given"))?)),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct NewQuestion{
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

// impl Question {
//     pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
//         Question {
//             id,
//             title,
//             content,
//             tags,
//         }
//     }
// }
