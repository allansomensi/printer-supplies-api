use std::str::FromStr;

use anyhow::{anyhow, Context};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, SqlitePool, Transaction};
use uuid::Uuid;

use domain::stock::models::toner::{CreateTonerError, DeleteTonerError, DeleteTonerRequest};
use domain::stock::models::toner::{CreateTonerRequest, Toner, TonerName};
use domain::stock::ports::StockRepository;

#[derive(Debug, Clone)]
pub struct Sqlite {
    pool: SqlitePool,
}

impl Sqlite {
    pub async fn new(path: &str) -> Result<Sqlite, anyhow::Error> {
        let pool = SqlitePool::connect_with(
            SqliteConnectOptions::from_str(path)
                .with_context(|| format!("Invalid database path {}", path))?
                .pragma("foreign_keys", "ON"),
        )
        .await
        .with_context(|| format!("Failed to open database at {}", path))?;

        Ok(Sqlite { pool })
    }

    async fn save_toner(
        &self,
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        name: &TonerName,
    ) -> Result<Uuid, sqlx::Error> {
        let id = Uuid::new_v4();
        let id_as_string = id.to_string();
        let name = &name.to_string();
        let query = sqlx::query!(
            "INSERT INTO toners (id, name) VALUES ($1, $2)",
            id_as_string,
            name,
        );
        tx.execute(query).await?;
        Ok(id)
    }

    async fn delete_toner(
        &self,
        tx: &mut Transaction<'_, sqlx::Sqlite>,
        id: &Uuid,
    ) -> Result<Uuid, sqlx::Error> {
        let id = &id.to_string();
        let query = sqlx::query!("DELETE FROM toners WHERE id = $1", id);
        tx.execute(query).await?;
        Ok(Uuid::from_str(id).unwrap())
    }
}

impl StockRepository for Sqlite {
    async fn create_toner(&self, req: &CreateTonerRequest) -> Result<Toner, CreateTonerError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("Failed to start SQLite transaction")?;

        let toner_id = self.save_toner(&mut tx, req.name()).await.map_err(|e| {
            if is_unique_constraint_violation(&e) {
                CreateTonerError::Duplicate {
                    name: req.name().clone(),
                }
            } else {
                anyhow!(e)
                    .context(format!("Failed to save toner with name {:?}", req.name()))
                    .into()
            }
        })?;

        tx.commit()
            .await
            .context("Failed to commit SQLite transaction")?;

        Ok(Toner::new(toner_id, req.name().clone()))
    }

    async fn delete_toner(&self, req: &DeleteTonerRequest) -> Result<Uuid, DeleteTonerError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .context("Failed to start SQLite transaction")?;

        let toner_id = self.delete_toner(&mut tx, req.id()).await.map_err(|e| {
            if exists(&e) {
                DeleteTonerError::NotFound {
                    id: req.id().clone(),
                }
            } else {
                anyhow!(e)
                    .context(format!("Failed to delete toner with id {:?}", req.id()))
                    .into()
            }
        })?;

        tx.commit()
            .await
            .context("Failed to commit SQLite transaction")?;

        Ok(toner_id)
    }
}

const UNIQUE_CONSTRAINT_VIOLATION_CODE: &str = "2067";
const ALREADY_EXISTS: &str = "409";

fn is_unique_constraint_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == UNIQUE_CONSTRAINT_VIOLATION_CODE {
                return true;
            }
        }
    }

    false
}

fn exists(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        if let Some(code) = db_err.code() {
            if code == ALREADY_EXISTS {
                return true;
            }
        }
    }

    false
}
