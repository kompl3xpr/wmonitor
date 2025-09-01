use crate::{utils::db::{GrayImg, RgbaImg}};

#[derive(Debug, sqlx::FromRow)]
pub struct Position {
    #[sqlx(rename = "pos_x")]
    pub x: i64,
    #[sqlx(rename = "pos_y")]
    pub y: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FiefChunk {
    pub id: i64,
    pub name: String,
    pub fief_id: i64,

    #[sqlx(flatten)]
    pub position: Position,

    pub img_ref: Option<RgbaImg>,
    pub img_mask: Option<GrayImg>,
    pub img_diff: Option<GrayImg>,
}

#[cfg(test)]
mod test {

    #[test]
    fn it_can_be_compiled() {
        let _ = <super::FiefChunk as sqlx::FromRow<crate::utils::db::CurrentRow>>::from_row;
    }
}