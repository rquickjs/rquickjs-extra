export {};

declare global {
  /**
   * Console interface for logging.
   *
   * Currently logs are only available in the backend logs.
   * See https://docs.caido.io/report_bug.html#1-backend-logs
   */
  type Console = {
    debug(message: any): void;
    log(message: any): void;
    warn(message: any): void;
    error(message: any): void;
  };
  var console: Console;
}
