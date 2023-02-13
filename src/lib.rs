// extern crate neon_serde2;
use neon::prelude::*;
use serde_json::Value;

pub fn convert_all_ints_to_strings(json: &str) -> Result<String, serde_json::Error> {
    fn convert_recursively(json: &mut Value) {
        match json {
            Value::Number(n) if n.is_u64() || n.is_i64() => {
                let m = 9007199254740991_f64;
                let n_num = n.as_f64().unwrap();
                if n_num > m {
                    *json = Value::String(n.to_string());
                }
            }
            Value::Array(a) => a.iter_mut().for_each(convert_recursively),
            Value::Object(o) => o.values_mut().for_each(convert_recursively),
            _ => (),
        }
    }

    serde_json::from_str(json).map(|mut v: Value| {
        convert_recursively(&mut v);
        v.to_string()
    })
}

fn parse(mut cx: FunctionContext) -> JsResult<JsObject> {
    let raw = cx.argument::<JsString>(0)?.value(&mut cx);
    let raw = convert_all_ints_to_strings(raw.as_str()).unwrap();
    let json = serde_json::from_str::<Value>(raw.as_str()).unwrap();
    let value = neon_serde3::to_value(&mut cx, &json).unwrap().downcast::<JsObject,_>(&mut cx).unwrap();
    Ok(value)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parse", parse)?;
    Ok(())
}
