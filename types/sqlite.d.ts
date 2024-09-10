declare module "sqlite" {
  export type Parameter = null | number | bigint | string | Uint8Array;
  export type Result = {
    changes: number;
    lastInsertRowid: number;
  };

  export type OpenOptions = {
    /**
     * The filename of the database. If the file does not exist, a new one will be created.
     */
    filename?: string | undefined;
    /**
     * If true, the database will be opened in-memory.
     * @default false
     */
    in_memory?: boolean | undefined;
    /**
     * If true, the database will use the WAL mode.
     * @default true
     */
    wal?: boolean | undefined;
  };

  /**
   * A SQLite database.
   *
   * The implementation uses a connection pool and is fully asynchronous.
   * Each connection will be spawned in a worker thread.
   *
   * @example
   * ```ts
   * const db = await open({ filename: "path/to/database.sqlite" });
   * await db.exec("CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);");
   * await db.exec("INSERT INTO test (name) VALUES ('foo');");
   */
  export class Database {
    /**
     * This method allows one or more SQL statements to be executed without returning any results.
     */
    exec(sql: string): Promise<void>;
    /**
     * Compiles a SQL statement into a {@link https://www.sqlite.org/c3ref/stmt.html prepared statement}.
     */
    prepare(sql: string): Statement;
  }

  /**
   * This class represents a single prepared statement. This class cannot be instantiated via its constructor.
   * Instead, instances are created via the database.prepare() method.
   */
  export class Statement {
    /**
     * This method executes a prepared statement and returns all results as an array of objects.
     * If the prepared statement does not return any results, this method returns an empty array.
     * The prepared statement {@link https://www.sqlite.org/c3ref/bind_blob.html parameters are bound} using the values in `params`.
     *
     * @param params The values to bind to the prepared statement. Named parameters are not supported.
     */
    all<T extends object = object>(...params: Parameter[]): Promise<T[]>;
    /**
     * This method executes a prepared statement and returns the first result as an object.
     * If the prepared statement does not return any results, this method returns undefined.
     * The prepared statement {@link https://www.sqlite.org/c3ref/bind_blob.html parameters are bound} using the values in params.
     *
     * @param params The values to bind to the prepared statement. Named parameters are not supported.
     */
    get<T extends object = object>(
      ...params: Parameter[]
    ): Promise<T | undefined>;
    /**
     * This method executes a prepared statement and returns an object summarizing the resulting changes.
     * The prepared statement {@link https://www.sqlite.org/c3ref/bind_blob.html parameters are bound} using the values in params.
     *
     * @param params The values to bind to the prepared statement. Named parameters are not supported.
     */
    run(...params: Parameter[]): Promise<Result>;
  }

  /**
   * Open a SQLite database.
   *
   * @param options The options to open the database.
   */
  export function open(options: OpenOptions): Promise<Database>;
}
