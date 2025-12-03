use rquickjs::{
    AsyncContext, AsyncRuntime, CatchResultExt, CaughtError, Ctx, FromJs, Function, Module, Result,
    async_with,
    function::IntoArgs,
    module::{Evaluated, ModuleDef},
    promise::MaybePromise,
};

pub fn test_with<F, R>(func: F)
where
    F: FnOnce(rquickjs::Ctx) -> R,
{
    let rt = rquickjs::Runtime::new().unwrap();
    let ctx = rquickjs::Context::full(&rt).unwrap();
    ctx.with(func);
}

pub async fn test_async_with<F>(func: F)
where
    F: for<'js> FnOnce(Ctx<'js>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + 'js>>
        + Send,
{
    let rt = AsyncRuntime::new().unwrap();
    let ctx = AsyncContext::full(&rt).await.unwrap();

    async_with!(ctx => |ctx| {
        func(ctx).await
    })
    .await;
}

pub async fn call_test<'js, T, A>(ctx: &Ctx<'js>, module: &Module<'js, Evaluated>, args: A) -> T
where
    T: FromJs<'js>,
    A: IntoArgs<'js>,
{
    call_test_err(ctx, module, args).await.unwrap()
}

pub async fn call_test_err<'js, T, A>(
    ctx: &Ctx<'js>,
    module: &Module<'js, Evaluated>,
    args: A,
) -> std::result::Result<T, CaughtError<'js>>
where
    T: FromJs<'js>,
    A: IntoArgs<'js>,
{
    module
        .get::<_, Function>("test")
        .catch(ctx)?
        .call::<_, MaybePromise>(args)
        .catch(ctx)?
        .into_future::<T>()
        .await
        .catch(ctx)
}

pub struct ModuleEvaluator;

impl ModuleEvaluator {
    pub async fn eval_js<'js>(
        ctx: Ctx<'js>,
        name: &str,
        source: &str,
    ) -> Result<Module<'js, Evaluated>> {
        let (module, module_eval) = Module::declare(ctx, name, source)?.eval()?;
        module_eval.into_future::<()>().await?;
        Ok(module)
    }

    pub async fn eval_rust<'js, M>(ctx: Ctx<'js>, name: &str) -> Result<Module<'js, Evaluated>>
    where
        M: ModuleDef,
    {
        let (module, module_eval) = Module::evaluate_def::<M, _>(ctx, name)?;
        module_eval.into_future::<()>().await?;
        Ok(module)
    }
}
