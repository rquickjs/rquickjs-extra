use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use crate::utils::result::ResultExt;
use either::Either;
use rquickjs::{
    function::{Opt, Rest},
    Ctx, Exception, Object, Result,
};

use super::{Argument, StatementRaw};

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct StatementSync {
    #[qjs(skip_trace)]
    raw: Option<StatementRaw>,
}

impl StatementSync {
    pub fn new() -> Self {
        todo!()
    }

    fn raw(&self, ctx: &Ctx<'_>) -> Result<&StatementRaw> {
        self.raw
            .as_ref()
            .ok_or_else(|| Exception::throw_message(ctx, "Statement has been finalized"))
    }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl StatementSync {
    fn get<'js>(
        &self,
        ctx: Ctx<'js>,
        named_or_anon_params: Either<Argument<'js>, Object<'js>>,
        anon_params: Rest<Argument<'js>>,
    ) -> Result<Option<Object<'js>>> {
        let raw = self.raw(&ctx)?;
        match named_or_anon_params {
            Either::Left(value) => {
                let mut index = 1;
                raw.bind(index, value).or_throw(&ctx)?;
                for value in anon_params.0 {
                    index += 1;
                    raw.bind(index, value).or_throw(&ctx)?;
                }
            }
            Either::Right(obj) => {
                todo!()
            }
        }
        todo!()
    }

    fn finalize(&mut self, ctx: Ctx<'_>) -> Result<()> {
        match self.raw.take() {
            Some(raw) => raw.finalize().or_throw(&ctx),
            None => Err(Exception::throw_message(
                &ctx,
                "Statement is already finalized",
            )),
        }
    }
}
