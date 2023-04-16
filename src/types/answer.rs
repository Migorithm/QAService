use super::question::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub corresponding_question: QuestionId,
}

impl Answer {
    pub fn new(id: i32, content: &str, corresponding_question: QuestionId) -> Self {
        Self {
            id: AnswerId(id),
            content: content.to_string(),
            corresponding_question,
        }
    }
}
#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct AnswerId(pub i32);
