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
        '"' => state.xml_attribute_info.update_state(
            &basic_info.current_key,
            XmlAttributesStages::Object(XmlAttributesObjectStages::Init),
        ),
        'n' => state.xml_attribute_info.update_state(
            &basic_info.current_key.clone(),
            XmlAttributesStages::Object(XmlAttributesObjectStages::NullValue("n".to_string())),
        ),
        _ => {
            state.xml_attribute_info.abort_xml_attributes();
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
            state.xml_attribute_info.update_state(
                &basic_info.current_key,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Value(
                    XmlAttributeKeyValueStages::Closed,
                )),
            );
        }
        _ => match "null".contains(new_str_val.as_str()) {
            true => {
                state.xml_attribute_info.update_state(
                    &basic_info.current_key.clone(),
                    XmlAttributesStages::Object(XmlAttributesObjectStages::NullValue(
                        "n".to_string(),
                    )),
                );
            }
            false => {
                state.xml_attribute_info.abort_xml_attributes();
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
            state
                .xml_attribute_info
                .update_xml_attribute_key(&&basic_info.current_key, attribute_key);
            state.xml_attribute_info.update_state(
                &basic_info.current_key,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Key(
                    XmlAttributeKeyValueStages::Closed,
                )),
            )
        }
        _ => match IS_ALLOWED_KEY_REGEX_EXPR.get().unwrap().is_match(&new_val) {
            true => state.xml_attribute_info.update_state(
                &basic_info.current_key,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Key(
                    XmlAttributeKeyValueStages::Open(new_val),
                )),
            ),
            false => {
                state.xml_attribute_info.abort_xml_attributes();
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
            state.xml_attribute_info.update_state(
                &basic_info.current_key,
                XmlAttributesStages::Object(XmlAttributesObjectStages::KeyValFieldSeparator),
            );
        }
        _ => state.xml_attribute_info.abort_xml_attributes(),
    }
}

pub fn object_attributes_stage_key_val_separator_case(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match char_val {
        '"' => {
            state.xml_attribute_info.update_state(
                &basic_info.current_key,
                XmlAttributesStages::Object(XmlAttributesObjectStages::Value(
                    XmlAttributeKeyValueStages::Open(String::new()),
                )),
            );
        }
        _ => state.xml_attribute_info.abort_xml_attributes(),
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
        '"' => state.xml_attribute_info.update_state(
            &basic_info.current_key,
            XmlAttributesStages::Object(XmlAttributesObjectStages::Value(
                XmlAttributeKeyValueStages::Closed,
            )),
        ),
        _ => {
            state.xml_attribute_info.update_state(
                &basic_info.current_key,
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
            state.xml_attribute_info.update_state(
                &basic_info.current_key,
                XmlAttributesStages::Object(XmlAttributesObjectStages::KeyValFieldSeparator),
            );
        }
        '}' => (),
        _ => state.xml_attribute_info.abort_xml_attributes(),
    }
}

pub fn object_attributes_stage_key_val_field_separator(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match char_val {
        '{' => {
            state.xml_attribute_info.update_state(
                &basic_info.current_key,
                XmlAttributesStages::Object(XmlAttributesObjectStages::KeyValFieldSeparator),
            );
        }
        _ => state.xml_attribute_info.abort_xml_attributes(),
    }
}
