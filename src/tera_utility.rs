use std::{borrow::Cow, collections::HashMap, sync::Arc};

use rocket_dyn_templates::{tera::Tera, Template};
use serde::Serialize;

use crate::{RocketOrbit, TeraError, TeraValue};

mod builtin {
    use super::*;

    fn js_global(map: &HashMap<String, TeraValue>) -> Result<String, TeraError> {
        let id = map
            .get("id")
            .ok_or_else(|| TeraError::msg("js_global: missing id argument"))?
            .as_str()
            .ok_or_else(|| TeraError::msg("js_global: id should be string"))?;
        let mut buf = format!("window[\"{id}\"]={{",);

        for (key, value) in map {
            if key == "id" {
                continue;
            }
            buf.push_str(
                format!(
                    "\"{key}\":\"{}\",",
                    value
                        .as_str()
                        .ok_or_else(|| {
                            TeraError::msg(format!(
                                "function - js_global: failed to stringify value {key}"
                            ))
                        })?
                        .replace("\\", "\\\\")
                        .replace("\n", "\\n")
                        .replace("\"", "\\\"")
                )
                .as_str(),
            );
        }
        buf.pop();
        buf.push_str("}");

        Ok(buf)
    }

    fn js_global_function(args: &HashMap<String, TeraValue>) -> Result<TeraValue, TeraError> {
        Ok(TeraValue::String(format!(
            "<script>{}</script>",
            js_global(args)?
        )))
    }
    fn js_global_filter(
        value: &TeraValue,
        _args: &HashMap<String, TeraValue>,
    ) -> Result<TeraValue, TeraError> {
        match value {
            TeraValue::Object(obj) => Ok(js_global_function(&obj.to_tera_hash_map())?),
            TeraValue::Array(arr) => {
                let mut buf = String::from("<script>");
                for obj in arr {
                    buf.push_str(
                        js_global(
                            &obj.as_object().ok_or_else(|| TeraError::msg("filter - js_global: value is array but not a array of objects"))?.to_tera_hash_map()
                        )?.as_str()
                    );
                    buf.push(';');
                }
                buf.push_str("</script>");
                Ok(TeraValue::String(buf))
            }
            _ => Err(TeraError::msg(
                "filter - js_global: value must be object or array of objects",
            )),
        }
    }

    fn wrap_script_filter(
        value: &TeraValue,
        _args: &HashMap<String, TeraValue>,
    ) -> Result<TeraValue, TeraError> {
        match value {
            TeraValue::String(script) => {
                Ok(TeraValue::String(format!("<script>{script}</script>")))
            }
            _ => Err(TeraError::msg("Inner value should be string")),
        }
    }

    fn newline_to_br_filter(
        value: &TeraValue,
        _args: &HashMap<String, TeraValue>,
    ) -> Result<TeraValue, TeraError> {
        match value {
            TeraValue::String(script) => Ok(TeraValue::String(script.replace("\n", "<br/>"))),
            _ => Err(TeraError::msg("Inner value should be string")),
        }
    }

    pub fn add_builtin(tera: &mut Tera) {
        tera.register_function("js_global", js_global_function);
        tera.register_filter("js_global", js_global_filter);
        tera.register_filter("wrap_script", wrap_script_filter);
        tera.register_filter("newline_to_br", newline_to_br_filter);
    }
}
pub use builtin::add_builtin;

pub trait ErrToTeraError<Value> {
    fn err_to_tera_error(self) -> Result<Value, TeraError>;
}
impl<T, E: ToString> ErrToTeraError<T> for Result<T, E> {
    fn err_to_tera_error(self) -> Result<T, TeraError> {
        self.map_err(|err| TeraError::msg(err.to_string()))
    }
}

pub trait TemplateToContent {
    fn template_to_content<S: Into<Cow<'static, str>>, C: Serialize>(
        &self,
        name: S,
        context: C,
    ) -> Result<Arc<str>, String>;
}
impl TemplateToContent for RocketOrbit {
    fn template_to_content<S: Into<Cow<'static, str>>, C: Serialize>(
        &self,
        name: S,
        context: C,
    ) -> Result<Arc<str>, String> {
        Ok(Arc::from(
            Template::show(self, name, context).ok_or_else(|| String::from("Failed to render"))?,
        ))
    }
}

pub trait ToTeraHashMap {
    fn to_tera_hash_map(self) -> HashMap<String, TeraValue>;
}
impl ToTeraHashMap for serde_json::Map<String, TeraValue> {
    fn to_tera_hash_map(self) -> HashMap<String, TeraValue> {
        let mut result = HashMap::with_capacity(self.len());
        for (key, value) in self {
            result.insert(key, value);
        }
        result
    }
}
impl ToTeraHashMap for &serde_json::Map<String, TeraValue> {
    fn to_tera_hash_map(self) -> HashMap<String, TeraValue> {
        let mut result = HashMap::with_capacity(self.len());
        for (key, value) in self {
            result.insert(key.clone(), value.clone());
        }
        result
    }
}
