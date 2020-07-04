use crate::prelude::*;

pub trait FunctionContextArgExt<'a> {
    fn arg_num(&mut self, i: i32) -> NeonResult<f64>;
    fn arg_str(&mut self, i: i32) -> NeonResult<String>;
    fn arg_func(&mut self, i: i32) -> NeonResult<Handle<'a, JsFunction>>;
    fn arg_cb(&mut self, i: i32) -> NeonResult<EventHandler>;
}

impl<'a> FunctionContextArgExt<'a> for FunctionContext<'a> {
    fn arg_num(&mut self, i: i32) -> NeonResult<f64> {
        Ok(self.argument::<JsNumber>(i)?.value())
    }

    fn arg_str(&mut self, i: i32) -> NeonResult<String> {
        Ok(self.argument::<JsString>(i)?.value())
    }

    fn arg_func(&mut self, i: i32) -> NeonResult<Handle<'a, JsFunction>> {
        Ok(self.argument::<JsFunction>(i)?)
    }

    fn arg_cb(&mut self, i: i32) -> NeonResult<EventHandler> {
        let this = self.undefined();
        let cb = self.arg_func(i)?;
        Ok(EventHandler::new(self, this, cb))
    }
}
