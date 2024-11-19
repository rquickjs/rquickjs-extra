// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// Copyright Emile Fugulin for modifications.
// SPDX-License-Identifier: Apache-2.0
// Source: https://github.com/awslabs/llrt/blob/07eb540a204dcdce44143220876630804f381ca6/llrt_utils/src/result.rs
use std::{fmt::Write, result::Result as StdResult};

use rquickjs::{Ctx, Exception, Result};

#[allow(dead_code)]
pub trait ResultExt<T> {
    fn or_throw_msg(self, ctx: &Ctx, msg: &str) -> Result<T>;
    fn or_throw_range(self, ctx: &Ctx, msg: Option<&str>) -> Result<T>;
    fn or_throw_type(self, ctx: &Ctx, msg: Option<&str>) -> Result<T>;
    fn or_throw(self, ctx: &Ctx) -> Result<T>;
}

impl<T, E: std::fmt::Display> ResultExt<T> for StdResult<T, E> {
    fn or_throw_msg(self, ctx: &Ctx, msg: &str) -> Result<T> {
        self.map_err(|e| {
            let mut message = String::with_capacity(100);
            message.push_str(msg);
            message.push_str(". ");
            write!(message, "{}", e).unwrap();
            Exception::throw_message(ctx, &message)
        })
    }

    fn or_throw_range(self, ctx: &Ctx, msg: Option<&str>) -> Result<T> {
        self.map_err(|e| {
            let mut message = String::with_capacity(100);
            if let Some(msg) = msg {
                message.push_str(msg);
                message.push_str(". ");
            }
            write!(message, "{}", e).unwrap();
            Exception::throw_range(ctx, &message)
        })
    }

    fn or_throw_type(self, ctx: &Ctx, msg: Option<&str>) -> Result<T> {
        self.map_err(|e| {
            let mut message = String::with_capacity(100);
            if let Some(msg) = msg {
                message.push_str(msg);
                message.push_str(". ");
            }
            write!(message, "{}", e).unwrap();
            Exception::throw_type(ctx, &message)
        })
    }

    fn or_throw(self, ctx: &Ctx) -> Result<T> {
        self.map_err(|err| Exception::throw_message(ctx, &err.to_string()))
    }
}

impl<T> ResultExt<T> for Option<T> {
    fn or_throw_msg(self, ctx: &Ctx, msg: &str) -> Result<T> {
        self.ok_or_else(|| Exception::throw_message(ctx, msg))
    }

    fn or_throw_range(self, ctx: &Ctx, msg: Option<&str>) -> Result<T> {
        self.ok_or_else(|| Exception::throw_range(ctx, msg.unwrap_or("")))
    }

    fn or_throw_type(self, ctx: &Ctx, msg: Option<&str>) -> Result<T> {
        self.ok_or_else(|| Exception::throw_type(ctx, msg.unwrap_or("")))
    }

    fn or_throw(self, ctx: &Ctx) -> Result<T> {
        self.ok_or_else(|| Exception::throw_message(ctx, "Value is not present"))
    }
}
