use schemars::{json_schema, JsonSchema};

use crate::HexColor;

#[cfg_attr(doc_cfg, doc(cfg(feature = "schemars")))]
impl JsonSchema for HexColor {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HexColor".into()
    }

    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        json_schema!({
            "description": "Hex color: #RGB, #RGBA, #RRGGBB or #RRGGBBAA",
            "type": "string",
            "pattern": "^#([a-fA-F0-9]{3,4}|[a-fA-F0-9]{6}|[a-fA-F0-9]{8})$"
        })
    }
}
