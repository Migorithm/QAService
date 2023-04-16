use super::question::QuestionId;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

impl Answer {
    pub fn new(id: i32, content: &str, question_id: QuestionId) -> Self {
        Self {
            id: AnswerId(id),
            content: content.to_string(),
            question_id,
        }
    }
}
#[derive(Default, Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct AnswerId(pub i32);
