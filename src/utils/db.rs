pub type CurrentDb = sqlx::Sqlite;
pub type CurrentRow = <CurrentDb as sqlx::Database>::Row;
pub type CurrentTypeInfo = <CurrentDb as sqlx::Database>::TypeInfo;

use sqlx::sqlite::SqliteQueryResult;

pub fn conv_create_result(result: Result<SqliteQueryResult, sqlx::Error>) -> anyhow::Result<bool> {
    match &result {
        Ok(r) => Ok(r.rows_affected() == 1),
        Err(sqlx::Error::Database(db_err)) => {
            if matches!(db_err.kind(), sqlx::error::ErrorKind::UniqueViolation) {
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

pub fn conv_fetch_one_result<T>(result: Result<T, sqlx::Error>) -> anyhow::Result<Option<T>> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(sqlx::Error::RowNotFound) => Ok(None),
        other_err => {
            other_err?;
            unreachable!();
        }
    }
}
