pub type CurrentDb = sqlx::Sqlite;
pub type CurrentRow = <CurrentDb as sqlx::Database>::Row;
pub type CurrentTypeInfo = <CurrentDb as sqlx::Database>::TypeInfo;