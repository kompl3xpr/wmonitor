CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS Fiefs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    check_interval_min INTEGER NOT NULL,
    last_check TEXT NOT NULL,
    skip_check_until TEXT NOT NULL,
    should_check_now BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS Members (
    user_id INTEGER NOT NULL,
    fief_id INTEGER NOT NULL,
    permissions INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, fief_id),
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (fief_id) REFERENCES Fiefs(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    fief_id INTEGER NOT NULL,
    pos_x INTEGER NOT NULL,
    pos_y INTEGER NOT NULL,
    img_ref BLOB,
    img_mask BLOB,
    img_diff BLOB,
    img_result BLOB,
    diff_count INTEGER NOT NULL,
    FOREIGN KEY (fief_id) REFERENCES Fiefs(id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_fiefs_name ON Fiefs (name);
CREATE INDEX IF NOT EXISTS idx_members_user_id ON Members (user_id);
CREATE INDEX IF NOT EXISTS idx_chunks_fief_id ON Chunks (fief_id);