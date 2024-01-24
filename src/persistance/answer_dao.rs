use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[async_trait]
pub trait AnswerDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
}

pub struct AnswerDaoImpl {
    db: PgPool,
}

impl AnswerDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswerDao for AnswerDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = sqlx::types::Uuid::parse_str(&answer.question_uuid).map_err(|_| {
            DBError::InvalidUUID(format!(
                "Could not parse answer UUID: {}",
                answer.question_uuid
            ))
        })?;

        let record = sqlx::query!(
            r#"
                INSERT INTO answer ( question_uuid, content )
                VALUES ( $1, $2 )
                RETURNING *
            "#,
            uuid,
            answer.content
        )
        .fetch_one(&self.db)
        .await
        .map_err(|e: sqlx::Error| match e {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    if code.eq(postgres_error_codes::FOREIGN_KEY_VIOLATION) {
                        return DBError::InvalidUUID(format!(
                            "Invalid question UUID: {}",
                            answer.question_uuid
                        ));
                    }
                }
                DBError::Other(Box::new(e))
            }
            e => DBError::Other(Box::new(e)),
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        todo!();
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        todo!();
    }
}
