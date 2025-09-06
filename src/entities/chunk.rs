#[derive(Debug, sqlx::FromRow)]
pub struct Position {
    #[sqlx(rename = "pos_x")]
    pub x: i64,
    #[sqlx(rename = "pos_y")]
    pub y: i64,
}



#[derive(Debug, sqlx::FromRow)]
pub struct ChunkWithoutImgs {
    pub id: i64,
    pub name: String,
    pub fief_id: i64,

    #[sqlx(flatten)]
    pub position: Position,

    pub diff_count: i64,
}

#[cfg(test)]
mod test {

    #[test]
    fn it_can_be_compiled() {
        let _ = <super::ChunkWithoutImgs as sqlx::FromRow<super::super::CurrentRow>>::from_row;
    }
}
