use crate::json_to_if::{models::IS_ALLOWED_KEY_REGEX_EXPR, State};

use super::models::{
    XmlAttributeKeyValueStages, XmlAttributesBasicInfo, XmlAttributesObjectStages,
    XmlAttributesStages,
};

pub fn object_attributes_stage_init(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match char_val {
        '"' => state.update_state(
            &basic_info,
            XmlAttributesStages::Object(XmlAttributesObjectStages::Init),
        ),
        'n' => state.update_state(
            &basic_info.clone(),
            XmlAttributesStages::Object(XmlAttributesObjectStages::NullValue("n".to_string())),
        ),
        _ => {
            state.abort_xml_attributes();
        }
    }
}

pub fn object_attributes_stage_null(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    curr_str_val: &String,
) {
    let new_str_val = format!("{}{}", curr_str_val, char_val);
    println!("current val is: {}", new_str_val);
    match new_str_val == "null" {
        true => {
            state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Value(
                    XmlAttributeKeyValueStages::Closed,
                )),
            );
        }
        _ => match "null".contains(new_str_val.as_str()) {
            true => {
                state.update_state(
                    &basic_info.clone(),
                    XmlAttributesStages::Object(XmlAttributesObjectStages::NullValue(
                        "n".to_string(),
                    )),
                );
            }
            false => {
                state.abort_xml_attributes();
            }
        },
    }
}

pub fn object_attributes_stage_key_open(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    attribute_key: &String,
) {
    let new_val = format!("{}{}", attribute_key, char_val);
    match char_val {
        '"' => {
            state.update_xml_attribute_key(attribute_key);
            state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Key(
                    XmlAttributeKeyValueStages::Closed,
                )),
            )
        }
        _ => match IS_ALLOWED_KEY_REGEX_EXPR.get().unwrap().is_match(&new_val) {
            true => state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Key(
                    XmlAttributeKeyValueStages::Open(new_val),
                )),
            ),
            false => {
                state.abort_xml_attributes();
            }
        },
    }
}

pub fn object_attributes_stage_key_closed(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match char_val {
        ':' => {
            state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::KeyValFieldSeparator),
            );
        }
        _ => state.abort_xml_attributes(),
    }
}

pub fn object_attributes_stage_key_val_separator_case(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match char_val {
        '"' => {
            state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Value(
                    XmlAttributeKeyValueStages::Open(String::new()),
                )),
            );
        }
        _ => state.abort_xml_attributes(),
    }
}

pub fn object_attributes_stage_value_open(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    curr_key: &String,
) {
    let new_val = format!("{}{}", curr_key, char_val);
    match char_val {
        '"' => state.update_state(
            &basic_info,
            XmlAttributesStages::Object(XmlAttributesObjectStages::Value(
                XmlAttributeKeyValueStages::Closed,
            )),
        ),
        _ => {
            state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Value(
                    XmlAttributeKeyValueStages::Open(new_val),
                )),
            );
        }
    }
}

pub fn object_attributes_stage_value_closed(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match char_val {
        ',' => {
            state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::KeyValFieldSeparator),
            );
        }
        '}' => (),
        _ => state.abort_xml_attributes(),
    }
}

pub fn object_attributes_stage_key_val_field_separator(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match char_val {
        '{' => {
            state.update_state(
                &basic_info,
                XmlAttributesStages::Object(XmlAttributesObjectStages::KeyValFieldSeparator),
            );
        }
        _ => state.abort_xml_attributes(),
    }
}
