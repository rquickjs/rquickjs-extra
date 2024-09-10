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
        .foreign_keys(options.foreign_keys)
        .page_size(options.page_size)
        .busy_timeout(Duration::from_millis(options.busy_timeout))
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
        .idle_timeout(options.idle_timeout.map(Duration::from_secs))
        .max_lifetime(Some(Duration::from_secs(options.max_lifetime)))
        .max_connections(options.max_connections)
        .min_connections(options.min_connections);

    let pool = pool_options.connect_with(connect_options).await.unwrap();
    Ok(Database::new(pool))
}

#[derive(Debug, Clone)]
pub struct OpenOptions {
    filename: Option<String>,
    in_memory: bool,
    wal: bool,
    page_size: u32,
    foreign_keys: bool,
    max_connections: u32,
    min_connections: u32,
    idle_timeout: Option<u64>,
    max_lifetime: u64,
    busy_timeout: u64,
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self {
            filename: None,
            in_memory: true,
            wal: true,
            page_size: 4096,
            foreign_keys: true,
            max_connections: 5,
            min_connections: 0,
            idle_timeout: None,
            max_lifetime: 60 * 60,
            busy_timeout: 5 * 1000,
        }
    }
}

impl<'js> FromJs<'js> for OpenOptions {
    fn from_js(_ctx: &Ctx<'js>, value: Value<'js>) -> Result<Self> {
        let default = OpenOptions::default();
        let obj = value.get::<Object<'js>>()?;
        let filename = obj.get("filename")?;
        let in_memory = obj.get("inMemory").unwrap_or(default.in_memory);
        let wal = obj.get("wal").unwrap_or(default.wal);
        let page_size = obj.get("pageSize").unwrap_or(default.page_size);
        let foreign_keys = obj.get("foreignKeys").unwrap_or(default.foreign_keys);
        let max_connections = obj.get("maxConnections").unwrap_or(default.max_connections);
        let min_connections = obj.get("minConnections").unwrap_or(default.min_connections);
        let idle_timeout = obj.get::<_, u64>("idleTimeout").ok();
        let max_lifetime = obj
            .get::<_, u64>("maxLifetime")
            .unwrap_or(default.max_lifetime);
        let busy_timeout = obj
            .get::<_, u64>("busyTimeout")
            .unwrap_or(default.busy_timeout);
        Ok(Self {
            filename,
            in_memory,
            wal,
            page_size,
            foreign_keys,
            max_connections,
            min_connections,
            idle_timeout,
            max_lifetime,
            busy_timeout,
        })
    }
}
