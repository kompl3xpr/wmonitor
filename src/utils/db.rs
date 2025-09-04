pub type CurrentDb = sqlx::Sqlite;
pub type CurrentRow = <CurrentDb as sqlx::Database>::Row;
pub type CurrentTypeInfo = <CurrentDb as sqlx::Database>::TypeInfo;

use sqlx::sqlite::SqliteQueryResult;

pub fn conv_create_result(result: Result<SqliteQueryResult, sqlx::Error>) -> anyhow::Result<bool> {
    match &result {
        Ok(r) => Ok(r.rows_affected() == 1),
        Err(sqlx::Error::Database(db_err)) => {
            if matches!(db_err.kind(), sqlx::error::ErrorKind::UniqueViolation | sqlx::error::ErrorKind::ForeignKeyViolation) {
                Ok(false)
            } else {
                result?;
                unreachable!();
            }
        }
        _ => {
            result?;
            unreachable!();
        }
    }
}
