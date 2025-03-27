use tauri_plugin_sql::{Migration, MigrationKind};

pub fn users_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_users_table",
        sql: r"
                CREATE TABLE IF NOT EXISTS twitch (
                    id TEXT,
                    username TEXT NOT NULL PRIMARY KEY,
                    avatar BLOB
                );
                
                CREATE TABLE IF NOT EXISTS youtube (
                    id TEXT,
                    username TEXT NOT NULL PRIMARY KEY,
                    avatar BLOB
                );
            ",
        kind: MigrationKind::Up,
    }]
}

pub fn feeds_migrations() -> Vec<Migration> {
    vec![
        Migration {
            version: 1,
            description: "create_feeds_table",
            sql: r"
                CREATE TABLE IF NOT EXISTS twitch (
                    username TEXT NOT NULL PRIMARY KEY,
                    started_at TEXT
                );

                CREATE TABLE IF NOT EXISTS youtube (
                    id TEXT NOT NULL PRIMARY KEY,
                    username TEXT NOT NULL,
                    title TEXT,
                    published_at TEXT,
                    view_count TEXT
                );
            ",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2,
            description: "youtube_published_integer_conversion",
            sql: r"
                PRAGMA foreign_keys=off;

                ALTER TABLE youtube RENAME TO old_youtube;

                CREATE TABLE youtube (
                    id TEXT NOT NULL PRIMARY KEY,
                    username TEXT NOT NULL,
                    title TEXT,
                    published_at INTEGER,
                    view_count TEXT
                );

                INSERT INTO youtube (id, username, title, published_at, view_count)
                    SELECT id, username, title, CAST(published_at AS INTEGER), view_count
                    FROM old_youtube;

                CREATE INDEX idx_published_at ON youtube (published_at);

                DROP TABLE old_youtube;
                PRAGMA foreign_keys=on;
            ",
            kind: MigrationKind::Up,
        },
    ]
}

pub fn emotes_migrations() -> Vec<Migration> {
    vec![Migration {
        version: 1,
        description: "create_emotes_table",
        sql: r"
                CREATE TABLE IF NOT EXISTS twitch (
                    username TEXT NOT NULL,
                    name TEXT NOT NULL,
                    url TEXT,
                    width INTEGER,
                    height INTEGER,
                    PRIMARY KEY (username, name)
                );
            ",
        kind: MigrationKind::Up,
    }]
}
