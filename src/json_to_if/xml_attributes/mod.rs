use self::{
    models::{
        XmlAttributeKeyValueStages, XmlAttributesArrayStages, XmlAttributesBasicInfo,
        XmlAttributesMapKey, XmlAttributesObjectStages, XmlAttributesType,
    },
    xml_attributes_array::{
        array_attributes_stage_key_closed, array_attributes_stage_key_open,
        array_attributes_stage_key_val_field_separator,
        array_attributes_stage_key_val_separator_case, array_attributes_stage_null,
        array_attributes_stage_object_end, array_attributes_stage_object_init,
        array_attributes_stage_object_separator, array_attributes_stage_value_closed,
        array_attributes_stage_value_open,
    },
    xml_attributes_object::{
        object_attributes_stage_init, object_attributes_stage_key_open,
        object_attributes_stage_key_val_field_separator,
        object_attributes_stage_key_val_separator_case, object_attributes_stage_null,
        object_attributes_stage_value_open,
    },
};

use super::State;

pub mod models;
pub mod xml_attributes_abort;
pub mod xml_attributes_array;
pub mod xml_attributes_end;
pub mod xml_attributes_marking;
pub mod xml_attributes_object;
pub mod xml_attributes_object_id;
pub mod xml_attributes_update;

fn xml_attributes_state_object_key_stages(
    char_val: &char,
    state: &mut State,
    key_val_stages: XmlAttributeKeyValueStages,
) {
    match key_val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            object_attributes_stage_key_open(char_val, state, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => todo!(),
    }
}

fn xml_attributes_state_object_val_stages(
    char_val: &char,
    state: &mut State,
    val_stages: XmlAttributeKeyValueStages,
) {
    match val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            object_attributes_stage_value_open(char_val, state, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => todo!(),
    }
}

fn xml_attributes_state_attributes_object(
    char_val: &char,
    state: &mut State,
    object_stages: XmlAttributesObjectStages,
) {
    match object_stages {
        XmlAttributesObjectStages::Init => object_attributes_stage_init(char_val, state),
        XmlAttributesObjectStages::Key(key_val_stages) => {
            xml_attributes_state_object_key_stages(char_val, state, key_val_stages)
        }
        XmlAttributesObjectStages::KeyValSeparator => {
            object_attributes_stage_key_val_separator_case(char_val, state)
        }
        XmlAttributesObjectStages::Value(val_stages) => {
            xml_attributes_state_object_val_stages(char_val, state, val_stages)
        }
        XmlAttributesObjectStages::KeyValFieldSeparator => {
            object_attributes_stage_key_val_field_separator(char_val, state)
        }
        XmlAttributesObjectStages::NullValue(curr_str_val) => {
            object_attributes_stage_null(char_val, state, &curr_str_val)
        }
    }
}

fn xml_attributes_state_array_key_stages(
    char_val: &char,
    state: &mut State,
    key_val_stages: XmlAttributeKeyValueStages,
) {
    match key_val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            array_attributes_stage_key_open(char_val, state, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => array_attributes_stage_key_closed(char_val, state),
    }
}

fn xml_attributes_state_array_val_stages(
    char_val: &char,
    state: &mut State,
    val_stages: XmlAttributeKeyValueStages,
) {
    match val_stages {
        XmlAttributeKeyValueStages::Open(str_val) => {
            array_attributes_stage_value_open(char_val, state, &str_val)
        }
        XmlAttributeKeyValueStages::Closed => array_attributes_stage_value_closed(char_val, state),
    }
}

fn xml_attributes_state_attributes_array(
    char_val: &char,
    state: &mut State,
    array_stages: XmlAttributesArrayStages,
) {
    match array_stages {
        XmlAttributesArrayStages::Init => array_attributes_stage_object_init(char_val, state),
        XmlAttributesArrayStages::ObjectInit => array_attributes_stage_object_init(char_val, state),
        XmlAttributesArrayStages::Key(xml_attribute_key_stage) => {
            xml_attributes_state_array_key_stages(char_val, state, xml_attribute_key_stage)
        }
        XmlAttributesArrayStages::KeyValSeparator => {
            array_attributes_stage_key_val_separator_case(char_val, state)
        }
        XmlAttributesArrayStages::Value(xml_attributes_val_stages) => {
            xml_attributes_state_array_val_stages(char_val, state, xml_attributes_val_stages)
        }
        XmlAttributesArrayStages::KeyValFieldSeparator => {
            array_attributes_stage_key_val_field_separator(char_val, state)
        }
        XmlAttributesArrayStages::ObjectEnd => array_attributes_stage_object_end(char_val, state),
        XmlAttributesArrayStages::ObjectSeparator => {
            array_attributes_stage_object_separator(char_val, state)
        }
        XmlAttributesArrayStages::NullValue(curr_str_val) => {
            array_attributes_stage_null(char_val, state, &curr_str_val)
        }
    }
}

fn xml_attributes_state_attributes(
    char_val: &char,
    state: &mut State,
    attributes_info: XmlAttributesBasicInfo,
) {
    match attributes_info.curr_stage {
        models::XmlAttributesStages::Array(array_stages) => {
            xml_attributes_state_attributes_array(char_val, state, array_stages)
        }
        models::XmlAttributesStages::Object(object_stages) => {
            xml_attributes_state_attributes_object(char_val, state, object_stages)
        }
    }
}

pub fn xml_attributes_check_state(char_val: &char, state: &mut State) {
    match state.xml_attributes.clone() {
        Some(attributes_info) => xml_attributes_state_attributes(char_val, state, attributes_info),
        None => (),
    }
}

pub fn get_attributes_type_mut<'a>(
    state: &'a mut State,
    xml_key: &String,
) -> Option<&'a mut XmlAttributesType> {
    let parent_index = state.fields.len() - 2;
    let last_index = state.fields.len() - 1;
    let last_field = state.fields[last_index.clone()].clone();
    let nesting_state = last_field.nesting_state.clone();
    let map_key = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: nesting_state,
    };
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .get_mut(&map_key)
}

fn get_attributes_type<'a>(state: &mut State, xml_key: &String) -> Option<XmlAttributesType> {
    let parent_index = state.fields.len() - 2;
    let last_index = state.fields.len() - 1;
    let last_field = state.fields[last_index.clone()].clone();
    let nesting_state = last_field.nesting_state.clone();
    let map_key = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: nesting_state,
    };
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .get(&map_key)
        .cloned()
}
