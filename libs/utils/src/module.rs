// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright Emile Fugulin for modifications.
// SPDX-License-Identifier: Apache-2.0
// Source: https://github.com/awslabs/llrt/blob/07eb540a204dcdce44143220876630804f381ca6/llrt_utils/src/module.rs
use rquickjs::{module::Exports, Ctx, Object, Result, Value};

pub fn export_default<'js, F>(ctx: &Ctx<'js>, exports: &Exports<'js>, f: F) -> Result<()>
where
    F: FnOnce(&Object<'js>) -> Result<()>,
{
    let default = Object::new(ctx.clone())?;
    f(&default)?;

    for name in default.keys::<String>() {
        let name = name?;
        let value: Value = default.get(&name)?;
        exports.export(name, value)?;
    }

    exports.export("default", default)?;

    Ok(())
}
