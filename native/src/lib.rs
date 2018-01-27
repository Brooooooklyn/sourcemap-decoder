#[macro_use]
extern crate neon;
extern crate sourcemap;

mod components;

use std::ops::Deref;
use std::ops::DerefMut;
use std::fs::File;
use neon::mem::Handle;
use neon::vm::{JsResult, This, FunctionCall, Lock};
use neon::js::{JsString, JsFunction, JsObject, Object, Value, JsNumber, JsArray, JsNull};
use neon::js::class::{JsClass, Class};
use components::parse::{parse};
use sourcemap::SourceMap;

pub struct SourceMapCache {
  pub sm: SourceMap,
}

trait CheckArgument<'a> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V>;
}

impl<'a, T: This> CheckArgument<'a> for FunctionCall<'a, T> {
  fn check_argument<V: Value>(&mut self, i: i32) -> JsResult<'a, V> {
    self.arguments.require(self.scope, i)?.check::<V>()
  }
}

declare_types! {
  pub class SourceMapDecoder for SourceMapCache {
    init(mut call) {
      let full_filename = call
        .check_argument::<JsString>(0)
        ?.value();
      let file_content = File::open(&full_filename).unwrap();
      let sm: SourceMap = SourceMap::from_reader(file_content).unwrap();
      Ok(SourceMapCache { sm })
    }

    method parse(mut call) {
      let line = call
        .check_argument::<JsNumber>(0)
        ?.value() as u32;
      let column = call
        .check_argument::<JsNumber>(1)
        ?.value() as u32;
      let scope = &mut *call.scope;
      let mut this = call.arguments.this(scope);
      let smc: &mut SourceMapCache = this.grab(|smc| smc );
      let mut js_array = JsArray::new(scope, 2);
      let js_array_deref = js_array.deref_mut();
      match parse(&smc.sm, line, column) {
        Ok((content, line)) => {
          let result_content = JsString::new(scope, &content).unwrap();
          let result_line = JsNumber::new(scope, line);
          js_array_deref.set(0, result_content).unwrap();
          js_array_deref.set(1, result_line).unwrap();
          Ok(js_array_deref.as_value(scope))
        },
        Err(e) => {
          println!("{:?}", e);
          Ok((JsNull::new().as_value(scope)))
        },
      }
    }
  }
}

register_module!(m, {
  let class: Handle<JsClass<SourceMapDecoder>> = try!(SourceMapDecoder::class(m.scope));
  let constructor: Handle<JsFunction<SourceMapDecoder>> = try!(class.constructor(m.scope));
  let exports = m.exports
    .check::<JsObject>()
    .unwrap();
  let deref_module = exports.deref();
  try!(deref_module.set("SourceMapDecoder", constructor));
  Ok(())
});
