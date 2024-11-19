use rquickjs::{Class, Ctx, Result};

pub use self::url_search_params::*;

mod url_search_params;

pub fn init(ctx: &Ctx<'_>) -> Result<()> {
    let globals = ctx.globals();

    Class::<URLSearchParams>::define(&globals)?;

    Ok(())
}
