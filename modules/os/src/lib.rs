// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright Emile Fugulin for modifications.
// SPDX-License-Identifier: Apache-2.0
use std::env;

use rquickjs::{
    Ctx, Exception, Result,
    module::{Declarations, Exports, ModuleDef},
    prelude::Func,
};
use rquickjs_extra_utils::{
    module::export_default,
    sysinfo::{get_arch, get_platform},
};

#[cfg(unix)]
use self::unix::{EOL, get_release, get_type, get_version};
#[cfg(windows)]
use self::windows::{EOL, get_release, get_type, get_version};

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

fn get_home_dir(ctx: Ctx<'_>) -> Result<String> {
    home::home_dir()
        .map(|val| val.to_string_lossy().into_owned())
        .ok_or_else(|| Exception::throw_message(&ctx, "Could not determine home directory"))
}

fn get_tmp_dir() -> String {
    env::temp_dir().to_string_lossy().to_string()
}

fn get_available_parallelism() -> usize {
    num_cpus::get()
}

pub struct OsModule;

impl ModuleDef for OsModule {
    fn declare(declare: &Declarations) -> Result<()> {
        declare.declare("arch")?;
        declare.declare("availableParallelism")?;
        declare.declare("EOL")?;
        declare.declare("platform")?;
        declare.declare("homedir")?;
        declare.declare("release")?;
        declare.declare("tmpdir")?;
        declare.declare("type")?;
        declare.declare("version")?;

        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            default.set("arch", Func::from(get_arch))?;
            default.set(
                "availableParallelism",
                Func::from(get_available_parallelism),
            )?;
            default.set("EOL", EOL)?;
            default.set("homedir", Func::from(get_home_dir))?;
            default.set("platform", Func::from(get_platform))?;
            default.set("release", Func::from(get_release))?;
            default.set("tmpdir", Func::from(get_tmp_dir))?;
            default.set("type", Func::from(get_type))?;
            default.set("version", Func::from(get_version))?;

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use rquickjs_extra_test::{ModuleEvaluator, call_test, test_async_with};

    use super::*;

    #[tokio::test]
    async fn test_type() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<OsModule>(ctx.clone(), "os")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { type } from 'os';

                        export async function test() {
                            return type()
                        }
                    "#,
                )
                .await
                .unwrap();

                let result = call_test::<String, _>(&ctx, &module, ()).await;

                assert!(result == "Linux" || result == "Windows_NT" || result == "Darwin");
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_release() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<OsModule>(ctx.clone(), "os")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { release } from 'os';

                        export async function test() {
                            return release()
                        }
                    "#,
                )
                .await
                .unwrap();

                let result = call_test::<String, _>(&ctx, &module, ()).await;

                assert!(!result.is_empty()); // Format is platform dependant
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_version() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<OsModule>(ctx.clone(), "os")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { version } from 'os';

                        export async function test() {
                            return version()
                        }
                    "#,
                )
                .await
                .unwrap();

                let result = call_test::<String, _>(&ctx, &module, ()).await;

                assert!(!result.is_empty()); // Format is platform dependant
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_available_parallelism() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<OsModule>(ctx.clone(), "os")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { availableParallelism } from 'os';

                        export async function test() {
                            return availableParallelism()
                        }
                    "#,
                )
                .await
                .unwrap();

                let result = call_test::<usize, _>(&ctx, &module, ()).await;

                assert!(result > 0);
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_eol() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<OsModule>(ctx.clone(), "os")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { EOL } from 'os';

                        export async function test() {
                            return EOL
                        }
                    "#,
                )
                .await
                .unwrap();

                let result = call_test::<String, _>(&ctx, &module, ()).await;
                assert!(result == EOL);
            })
        })
        .await;
    }

    #[tokio::test]
    async fn test_arch() {
        test_async_with(|ctx| {
            Box::pin(async move {
                ModuleEvaluator::eval_rust::<OsModule>(ctx.clone(), "os")
                    .await
                    .unwrap();

                let module = ModuleEvaluator::eval_js(
                    ctx.clone(),
                    "test",
                    r#"
                        import { arch } from 'os';

                        export async function test() {
                            return arch()
                        }
                    "#,
                )
                .await
                .unwrap();

                let result = call_test::<String, _>(&ctx, &module, ()).await;

                assert!(!result.is_empty()); // Format is platform dependant
            })
        })
        .await;
    }
}
