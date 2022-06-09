use anyhow::Result;
use duckdb::DuckdbConnectionManager;

type Pool = r2d2::Pool<DuckdbConnectionManager>;
type Conn = r2d2::PooledConnection<DuckdbConnectionManager>;

#[derive(Clone)]
pub struct Duck(Pool);

impl Duck {
    pub fn setup(path: &str) -> Result<Self> {
        let manager = DuckdbConnectionManager::file(path)?;
        let pool = r2d2::Pool::new(manager)?;

        Ok(Self(pool))
    }

    pub fn memory() -> Result<Self> {
        let manager = DuckdbConnectionManager::memory()?;
        let pool = r2d2::Pool::new(manager)?;

        Ok(Self(pool))
    }

    pub fn get(&self) -> Result<Conn> {
        Ok(self.0.get()?)
    }

    pub fn migrate(&self) -> Result<()> {
        let mut conn = self.get()?;
        let tx = conn.transaction()?;
        let version = 1;

        if version <= 1 {
            tx.execute_batch(
                r"
                CREATE TABLE IF NOT EXISTS item (
                    id INTEGER NOT NULL,
                    original VARCHAR NOT NULL,
                    descendants INTEGER,
                    username VARCHAR,
                    score INTEGER,
                    title VARCHAR,
                    url VARCHAR,
                    body VARCHAR,
                    ts TIMESTAMP
                );

                CREATE TABLE IF NOT EXISTS item_rank (
                    id INTEGER NOT NULL,
                    rank INTEGER NOT NULL, 
                    category VARCHAR NOT NULL, 
                    ts TIMESTAMP NOT NULL,
                    PRIMARY KEY (id, category, ts)
                );

                CREATE TABLE IF NOT EXISTS item_score (
                    id INTEGER NOT NULL,
                    ts TIMESTAMP NOT NULL,
                    score INTEGER NOT NULL,
                    PRIMARY KEY (id, ts)
                );

                CREATE TABLE IF NOT EXISTS item_bookmark (
                    id INTEGER NOT NULL,
                    user_id VARCHAR NOT NULL,
                    ts TIMESTAMP NOT NULL,
                    PRIMARY KEY (id, user_id)
                );

                CREATE TABLE IF NOT EXISTS item_list (
                    category VARCHAR NOT NULL,
                    ids VARCHAR NOT NULL, -- Serialized vec of ids
                    ts TIMESTAMP,
                    PRIMARY KEY (category)
                );

                CREATE TABLE IF NOT EXISTS hn_user (
                    id VARCHAR NOT NULL,
                    created INTEGER NOT NULL,
                    karma INTEGER NOT NULL,
                    delay INTEGER,
                    about VARCHAR,
                    submitted VARCHAR NOT NULL,
                    PRIMARY KEY (id)
                );

                CREATE TABLE IF NOT EXISTS config (
                    key VARCHAR,
                    value TEXT,
                    PRIMARY KEY (key)
                );
                ",
            )?;
        }

        tx.commit()?;

        Ok(())
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use duckdb::params;

    pub fn setup() -> Duck {
        let db = Duck::memory().unwrap();
        db.migrate().unwrap();
        db
    }

    fn setup_conn() -> Conn {
        let db = Duck::memory().unwrap();
        db.migrate().unwrap();
        let conn = db.get().unwrap();
        conn
    }

    #[test]
    fn test_setup() {
        let got = Duck::setup("test.db").is_ok();
        let want = true;
        assert_eq!(got, want);
    }

    #[test]
    fn migrate_v1_store_item_list() {
        let conn = setup_conn();

        let got = conn
            .execute(
                r#"
                    INSERT INTO item_list (category, ids, ts)
                    VALUES 
                    (?1, ?2, ?3)
                "#,
                params!["top", "[1, 2, 3]", "2020-01-01T00:00:00Z"],
            )
            .is_ok();
        let want = true;

        assert_eq!(got, want);
    }

    #[test]
    fn migrate_v1_insert_item() {
        let conn = setup_conn();

        let got = conn
            .execute(
                r#"
                    INSERT INTO item (id, original, descendants, username, score, title, url, body, ts)
                    VALUES 
                    (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                "#,
                params![
                    1,
                    "",
                    0,
                    Some("username"),
                    Some(1),
                    Some("title"),
                    Some("url"),
                    Some("body"),
                    Some("2020-01-01T00:00:00Z")
                ],
            )
            .is_ok();
        let want = true;

        assert_eq!(got, want);
    }

    #[test]
    fn migrate_v1_insert_item_rank() {
        let conn = setup_conn();
        let got = conn
            .execute(
                r#"
                    INSERT INTO item_rank (id, rank, category, ts)
                    VALUES
                    (?1, ?2, ?3, ?4)
                "#,
                params![1, 1, "top", "2020-01-01T00:00:00Z",],
            )
            .unwrap();
        let want = 1;
        assert_eq!(got, want);
    }

    #[test]
    fn migrate_v1_insert_item_score() {
        let db = Duck::memory().unwrap();
        db.migrate().unwrap();
        let conn = db.get().unwrap();

        let got = conn
            .execute(
                r#"
                    INSERT INTO item_score (id, score, ts)
                    VALUES
                    (?1, ?2, ?3)
                "#,
                params![1, 1, "2020-01-01T00:00:00Z"],
            )
            .unwrap();

        let want = 1;
        assert_eq!(got, want);
    }

    #[test]
    fn migrate_v1_insert_item_bookmark() {
        let conn = setup_conn();
        let got = conn
            .execute(
                r#"
                    INSERT INTO item_bookmark (id, user_id, ts)
                    VALUES
                    (?1, ?2, ?3)
                "#,
                params![1, "user 1", "2020-01-01T00:00:00Z"],
            )
            .unwrap();

        let want = 1;
        assert_eq!(got, want);
    }

    #[test]
    fn migrate_v1_insert_config() {
        let conn = setup_conn();
        let got = conn
            .execute(
                r#"
                    INSERT INTO config (key, value)
                    VALUES
                    (?1, ?2)
                "#,
                params!["some key", "some value"],
            )
            .unwrap();

        let want = 1;
        assert_eq!(got, want);
    }

    #[test]
    fn migrate_v1_insert_user() {
        let conn = setup_conn();
        let got = conn
            .execute(
                r#"
                    INSERT INTO hn_user (id, created, karma, delay, about, submitted)
                    VALUES
                    (?1, ?2, ?3, ?4, ?5, ?6)
                "#,
                params!["some user", 0, 0, Some(0), Some("about me"), "[0]"],
            )
            .unwrap();

        let want = 1;
        assert_eq!(got, want);
    }
}
