pub struct ChunkStatus {
    pub pos_x: u16,
    pub pos_y: u16,
    pub img_current: (),
    pub last_update: chrono::DateTime<chrono::Utc>,
}