CREATE TABLE IF NOT EXISTS Users (
    id INTEGER PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS Fiefs (
    id INTEGER PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    check_duration_ms INTEGER NOT NULL,
    last_check TEXT,
    skip_check_until TEXT
);

CREATE TABLE IF NOT EXISTS FiefMembers (
    fief_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    permissions INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (fief_id, user_id),
    FOREIGN KEY (fief_id) REFERENCES Fiefs(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES Users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS FiefChunks (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    fief_id INTEGER NOT NULL,
    pos_x INTEGER NOT NULL,
    pos_y INTEGER NOT NULL,
    img_ref BLOB,
    img_mask BLOB,
    img_diff BLOB,
    FOREIGN KEY (fief_id) REFERENCES Fiefs(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Events (
    id INTEGER PRIMARY KEY,
    date TEXT NOT NULL,
    value TEXT
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_username ON Users (username);
CREATE UNIQUE INDEX IF NOT EXISTS idx_fiefs_name ON Fiefs (name);
CREATE INDEX IF NOT EXISTS idx_fief_members_user_id ON FiefMembers (user_id);
CREATE INDEX IF NOT EXISTS idx_fief_chunks_fief_id ON FiefChunks (fief_id);
CREATE INDEX IF NOT EXISTS idx_events_date ON Events (date);