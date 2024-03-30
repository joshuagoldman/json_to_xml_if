use self::{
    models::{
        XmlAttributeKeyValueStages, XmlAttributeState, XmlAttributesArrayStages,
        XmlAttributesBasicInfo, XmlAttributesObjectStages,
    },
    xml_attributes_array::{
        array_attributes_stage_key_closed, array_attributes_stage_key_open,
        array_attributes_stage_key_val_field_separator,
        array_attributes_stage_key_val_separator_case, array_attributes_stage_object_end,
        array_attributes_stage_object_init, array_attributes_stage_object_separator,
        array_attributes_stage_value_closed, array_attributes_stage_value_open,
    },
    xml_attributes_object::{
        object_attributes_stage_init, object_attributes_stage_key_open,
        object_attributes_stage_key_val_field_separator,
        object_attributes_stage_key_val_separator_case, object_attributes_stage_value_open,
    },
};

use super::State;

pub mod models;
pub mod xml_attributes_array;
pub mod xml_attributes_object;

fn xml_attributes_state_object_key_stages(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    key_val_stages: XmlAttributeKeyValueStages,
) {
    match key_val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            object_attributes_stage_key_open(char_val, state, basic_info, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => todo!(),
    }
}

fn xml_attributes_state_object_val_stages(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    val_stages: XmlAttributeKeyValueStages,
) {
    match val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            object_attributes_stage_value_open(char_val, state, basic_info, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => todo!(),
    }
}

fn xml_attributes_state_attributes_object(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    object_stages: XmlAttributesObjectStages,
) {
    match object_stages {
        XmlAttributesObjectStages::Init => {
            object_attributes_stage_init(char_val, state, basic_info)
        }
        XmlAttributesObjectStages::Key(key_val_stages) => {
            xml_attributes_state_object_key_stages(char_val, state, basic_info, key_val_stages)
        }
        XmlAttributesObjectStages::KeyValSeparator => {
            object_attributes_stage_key_val_separator_case(char_val, state, basic_info)
        }
        XmlAttributesObjectStages::Value(val_stages) => {
            xml_attributes_state_object_val_stages(char_val, state, basic_info, val_stages)
        }
        XmlAttributesObjectStages::KeyValFieldSeparator => {
            object_attributes_stage_key_val_field_separator(char_val, state, basic_info)
        }
    }
}

fn xml_attributes_state_array_key_stages(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    key_val_stages: XmlAttributeKeyValueStages,
) {
    match key_val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            array_attributes_stage_key_open(char_val, state, basic_info, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => {
            array_attributes_stage_key_closed(char_val, state, basic_info)
        }
    }
}

fn xml_attributes_state_array_val_stages(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    val_stages: XmlAttributeKeyValueStages,
) {
    match val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            array_attributes_stage_value_open(char_val, state, basic_info, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => {
            array_attributes_stage_value_closed(char_val, state, basic_info)
        }
    }
}

fn xml_attributes_state_attributes_array(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    array_stages: XmlAttributesArrayStages,
) {
    match array_stages {
        XmlAttributesArrayStages::Init => {
            array_attributes_stage_object_init(char_val, state, basic_info)
        }
        XmlAttributesArrayStages::ObjectInit => {
            array_attributes_stage_object_init(char_val, state, basic_info)
        }
        XmlAttributesArrayStages::Key(xml_attribute_key_stage) => {
            xml_attributes_state_array_key_stages(
                char_val,
                state,
                basic_info,
                xml_attribute_key_stage,
            )
        }
        XmlAttributesArrayStages::KeyValSeparator => {
            array_attributes_stage_key_val_separator_case(char_val, state, basic_info)
        }
        XmlAttributesArrayStages::Value(xml_attributes_val_stages) => {
            xml_attributes_state_array_val_stages(
                char_val,
                state,
                basic_info,
                xml_attributes_val_stages,
            )
        }
        XmlAttributesArrayStages::KeyValFieldSeparator => {
            array_attributes_stage_key_val_field_separator(char_val, state, basic_info)
        }
        XmlAttributesArrayStages::ObjectEnd => {
            array_attributes_stage_object_end(char_val, state, basic_info)
        }
        XmlAttributesArrayStages::ObjectSeparator => {
            array_attributes_stage_object_separator(char_val, state, basic_info)
        }
    }
}

fn xml_attributes_state_attributes(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    attributes_info: XmlAttributesBasicInfo,
) {
    match attributes_info.curr_stage {
        models::XmlAttributesStages::Array(array_stages) => {
            xml_attributes_state_attributes_array(char_val, state, basic_info, array_stages)
        }
        models::XmlAttributesStages::Object(object_stages) => {
            xml_attributes_state_attributes_object(char_val, state, basic_info, object_stages)
        }
    }
}

fn xml_attributes_check_state(
    char_val: &char,
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) {
    match state.xml_attribute_info.current_state.clone() {
        XmlAttributeState::NoAttributes => (),
        XmlAttributeState::Attributes(attributes_info) => {
            xml_attributes_state_attributes(char_val, state, basic_info, attributes_info)
        }
    }
}
