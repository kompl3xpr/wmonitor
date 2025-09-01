#[derive(Debug, sqlx::FromRow)]
pub struct FiefChunk {
    pub id: i32,
    pub name: String,
    pub fief_id: i32,
    pub pos_x: u16,
    pub pos_y: u16,
    pub img_ref: (),
    pub img_mask: (),
    pub img_diff: (),
}