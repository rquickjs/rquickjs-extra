use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    time::Duration,
};

use either::Either;
use rquickjs::{Ctx, FromJs, Null, Object, Result, Value};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

use crate::utils::result::ResultExt;

use super::Database;

static IN_MEMORY_DB_SEQ: AtomicUsize = AtomicUsize::new(0);

pub async fn open(ctx: Ctx<'_>, options: OpenOptions) -> Result<Database> {
    let mut connect_options = SqliteConnectOptions::new();
    connect_options = connect_options
        .foreign_keys(options.foreign_keys)
        .page_size(options.page_size)
        .busy_timeout(options.busy_timeout)
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
        .idle_timeout(options.idle_timeout)
        .max_lifetime(options.max_lifetime)
        .max_connections(options.max_connections)
        .min_connections(options.min_connections);

    let pool = pool_options
        .connect_with(connect_options)
        .await
        .or_throw_msg(&ctx, "Unable to open database")?;
    Ok(Database::new(pool))
}

#[derive(Debug, Clone)]
pub struct OpenOptions {
    filename: Option<PathBuf>,
    in_memory: bool,
    wal: bool,
    page_size: u32,
    foreign_keys: bool,
    max_connections: u32,
    min_connections: u32,
    idle_timeout: Option<Duration>,
    max_lifetime: Option<Duration>,
    busy_timeout: Duration,
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
            max_lifetime: Some(Duration::from_secs(60 * 60)),
            busy_timeout: Duration::from_millis(5 * 1000),
        }
    }
}

impl<'js> FromJs<'js> for OpenOptions {
    fn from_js(_ctx: &Ctx<'js>, value: Value<'js>) -> Result<Self> {
        let default = OpenOptions::default();
        let obj = value.get::<Object<'js>>()?;
        let filename = obj.get::<_, String>("filename").map(PathBuf::from).ok();
        let in_memory = obj.get::<_, bool>("inMemory").unwrap_or(default.in_memory);
        let wal = obj.get::<_, bool>("wal").unwrap_or(default.wal);
        let page_size = obj.get::<_, u32>("pageSize").unwrap_or(default.page_size);
        let foreign_keys = obj
            .get::<_, bool>("foreignKeys")
            .unwrap_or(default.foreign_keys);
        let max_connections = obj
            .get::<_, u32>("maxConnections")
            .unwrap_or(default.max_connections);
        let min_connections = obj
            .get::<_, u32>("minConnections")
            .unwrap_or(default.min_connections);
        let idle_timeout = obj
            .get::<_, Option<u64>>("idleTimeout")?
            .map(Duration::from_secs);
        let max_lifetime =
            obj.get::<_, Either<Option<u64>, Null>>("maxLifetime")
                .map(|e| match e {
                    Either::Left(s) => s.map(Duration::from_secs).or(default.max_lifetime),
                    Either::Right(_) => None,
                })?;
        let busy_timeout = obj
            .get::<_, u64>("busyTimeout")
            .map(Duration::from_secs)
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
