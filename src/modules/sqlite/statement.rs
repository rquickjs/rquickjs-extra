use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct Statement {}

impl Statement {
    pub fn new() -> Self {
        todo!()
    }
}
