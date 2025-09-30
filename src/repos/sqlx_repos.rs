mod chunk;
pub use chunk::SqlxChunkRepo;

mod fief;
pub use fief::SqlxFiefRepo;

mod user;
use sqlx::sqlite::SqliteQueryResult;
pub use user::SqlxUserRepo;
pub fn conv_create_result<Id>(
    result: Result<SqliteQueryResult, sqlx::Error>,
) -> Result<Option<Id>, sqlx::Error>
where
    Id: From<i64>,
{
    match &result {
        Ok(r) => Ok(Some(r.last_insert_rowid().into())),
        Err(sqlx::Error::Database(_)) => Ok(None),
        _ => Err(result.unwrap_err()),
    }
}
