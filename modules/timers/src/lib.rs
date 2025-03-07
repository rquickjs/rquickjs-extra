use std::sync::Arc;
use std::time::Duration;

use rquickjs::{
    class::Trace,
    function::Opt,
    prelude::Func,
    JsLifetime, {Class, Ctx, Function, Result},
};
use tokio::sync::Notify;

const TARGET: &str = "timers";

#[derive(Trace, JsLifetime)]
#[rquickjs::class]
struct Timeout {
    #[qjs(skip_trace)]
    abort: Arc<Notify>,
}

fn clear_timeout(_ctx: Ctx<'_>, timeout: Class<Timeout>) -> Result<()> {
    timeout.borrow().abort.notify_one();
    Ok(())
}

fn set_timeout_interval<'js>(
    ctx: Ctx<'js>,
    cb: Function<'js>,
    msec: Option<u64>,
    is_interval: bool,
) -> Result<Class<'js, Timeout>> {
    let mut msecs = msec.unwrap_or(0);
    if msecs < 4 {
        msecs = 4;
    }
    let duration = Duration::from_millis(msecs);

    let abort = Arc::new(Notify::new());
    let abort_ref = abort.clone();

    ctx.spawn(async move {
        loop {
            let abort = abort_ref.clone();
            let aborted;

            let mut interval = tokio::time::interval(duration);
            interval.tick().await; // Skip the first tick
            tokio::select! {
                _ = abort.notified() => {
                    aborted = true;
                },
                _ = interval.tick() => {
                    aborted = false;
                }
            }

            if aborted {
                break;
            }

            if let Err(err) = cb.call::<(), ()>(()) {
                log::error!(target: TARGET, "Failed to call timeout/interval callback: {}", err);
                break;
            }

            if !is_interval {
                break;
            }
        }
        drop(cb);
        drop(abort_ref);
    });

    Class::instance(ctx, Timeout { abort })
}

fn set_timeout<'js>(
    ctx: Ctx<'js>,
    cb: Function<'js>,
    msec: Opt<u64>,
) -> Result<Class<'js, Timeout>> {
    set_timeout_interval(ctx, cb, msec.0, false)
}

fn set_interval<'js>(
    ctx: Ctx<'js>,
    cb: Function<'js>,
    msec: Opt<u64>,
) -> Result<Class<'js, Timeout>> {
    set_timeout_interval(ctx, cb, msec.0, true)
}

fn set_immediate(cb: Function) -> Result<()> {
    cb.defer::<()>(())?;
    Ok(())
}

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    globals.set("setTimeout", Func::from(set_timeout))?;
    globals.set("clearTimeout", Func::from(clear_timeout))?;
    globals.set("setInterval", Func::from(set_interval))?;
    globals.set("clearInterval", Func::from(clear_timeout))?;
    globals.set("setImmediate", Func::from(set_immediate))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use futures::FutureExt;
    use rquickjs::promise::Promise;
    use rquickjs::CatchResultExt;
    use rquickjs_extra_test::test_async_with;

    use super::*;

    #[tokio::test]
    async fn test_set_timeout() {
        test_async_with(|ctx| {
            async move {
                init(&ctx).unwrap();

                let result = ctx
                    .eval::<Promise, _>(
                        r#"

                        (async function(){
                            return new Promise((resolve, reject) => {
                                setTimeout(() => {
                                    resolve("Hello World");
                                }, 100);
                            });
                        })()
                    "#,
                    )
                    .catch(&ctx)
                    .unwrap()
                    .into_future::<String>()
                    .await
                    .catch(&ctx)
                    .unwrap();

                assert_eq!("Hello World", result);
            }
            .boxed_local()
        })
        .await
    }

    #[tokio::test]
    async fn test_set_interval() {
        test_async_with(|ctx| {
            async move {
                init(&ctx).unwrap();

                let result = ctx
                    .eval::<Promise, _>(
                        r#"

                        (async function(){
                            return new Promise((resolve, reject) => {
                                let count = 0;
                                setInterval(() => {
                                    if (++count === 3) {
                                        resolve(count);
                                    }
                                }, 100);
                            });
                        })()
                    "#,
                    )
                    .catch(&ctx)
                    .unwrap()
                    .into_future::<u32>()
                    .await
                    .catch(&ctx)
                    .unwrap();

                assert_eq!(3, result);
            }
            .boxed_local()
        })
        .await
    }
}
