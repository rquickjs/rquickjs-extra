use std::{
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use rquickjs::{Ctx, FromJs, Object, Result, Value};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use super::Database;

static IN_MEMORY_DB_SEQ: AtomicUsize = AtomicUsize::new(0);

pub async fn open(options: OpenOptions) -> Result<Database> {
    let mut connect_options = SqliteConnectOptions::new();
    connect_options = connect_options
        .foreign_keys(true)
        .thread_name(|id| format!("quickjs-sqlite-worker-{id}"));
    if let Some(filename) = options.filename {
        connect_options = connect_options.filename(filename).create_if_missing(true);
    }
    if options.in_memory {
        let seqno = IN_MEMORY_DB_SEQ.fetch_add(1, Ordering::Relaxed);
        connect_options = connect_options
            .filename(format!("file:sqlite-in-memory-{seqno}"))
            .in_memory(true)
            .shared_cache(true);
    }
    if options.wal {
        connect_options = connect_options.journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    }

    let mut pool_options = SqlitePoolOptions::new();
    pool_options = pool_options
        .idle_timeout(None)
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .max_connections(5)
        .min_connections(0);

    let pool = pool_options.connect_with(connect_options).await.unwrap();
    Ok(Database::new(pool))
}

pub struct OpenOptions {
    filename: Option<String>,
    in_memory: bool,
    wal: bool,
}

impl<'js> FromJs<'js> for OpenOptions {
    fn from_js(_ctx: &Ctx<'js>, value: Value<'js>) -> Result<Self> {
        let obj = value.get::<Object<'js>>()?;
        let filename = obj.get("filename")?;
        let in_memory = obj.get("inMemory").unwrap_or(false);
        let wal = obj.get("wal").unwrap_or(true);
        Ok(Self {
            filename,
            in_memory,
            wal,
        })
    }
}
