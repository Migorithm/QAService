use super::models::{Question, QuestionId};
use std::{borrow::BorrowMut, collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    pub(crate) questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("Can't read questions.json")
    }
}
