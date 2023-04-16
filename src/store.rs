use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::question;
use crate::types::{
    answer::{Answer, AnswerId},
    question::{NewQuestion, Question, QuestionId},
};

use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't estabilish DB connection: {}", e),
        };
        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<i32>,
        offset: i32,
    ) -> Result<Vec<Question>, Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
    pub async fn get_question(&self, id: i32) -> Result<Question, Error> {
        match sqlx::query("SELECT * FROM questions LIMIT $1")
            .bind(1)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(question) => Ok(question),
            Err(_) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::QuestionNotFound)
            }
        }
    }

    pub async fn get_answers(&self, corresponding_question: i32) -> Result<Vec<Answer>, Error> {
        match sqlx::query("SELECT * FROM answers WHERE corresponding_question = $1")
            .bind(corresponding_question)
            .map(|row: PgRow| Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                corresponding_question: QuestionId(row.get("corresponding_question")),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(answers) => Ok(answers),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn add_question(&self, new_question: NewQuestion) -> Result<Question, Error> {
        match sqlx::query(
            "INSERT INTO questions (title,content,tags) 
                  VALUES ($1, $2, $3)
                  RETURNING id, title, content, tags",
        )
        .bind(new_question.title)
        .bind(new_question.content)
        .bind(new_question.tags)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn update_question(&self, id: i32, question: Question) -> Result<Question, Error> {
        match sqlx::query(
            "UPDATE questions 
                 SET title = $1,
                    content = $2,
                    tags = $3
                 WHERE id = $4 
                 RETURNING id,title,content,tags",
        )
        .bind(question.title)
        .bind(question.content)
        .bind(question.tags)
        .bind(id)
        .map(|row: PgRow| Question {
            id: QuestionId(row.get("id")),
            title: row.get("title"),
            content: row.get("content"),
            tags: row.get("tags"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(question) => Ok(question),
            Err(e) => Err(Error::DatabaseQueryError(e)),
        }
    }

    pub async fn delete_question(&self, id: i32) -> Result<bool, Error> {
        match sqlx::query(
            "DELETE FROM questions WHERE id = $1",
        )
        .bind(id)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
