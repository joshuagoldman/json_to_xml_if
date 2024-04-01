use crate::json_to_if::{
    models::NestingState,
    state::State,
    xml_attributes::models::{XmlAttributesMapKey, XmlAttributesType},
};

use super::{
    get_attributes_type,
    models::{XmlAttributeArrayinfo, XmlAttributeNoAttributeInfo, XmlAttributeObjectInfo},
};

fn abort_attributes_case_obj(
    state: &mut State,
    xml_key: &String,
    xml_attr_info: XmlAttributeObjectInfo,
) {
    let parent_index = state.fields.len() - 2;
    let map_key_obj = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonObjectNestinState,
    };
    let attr_new_info = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
        unique_key_ids: vec![xml_attr_info.unique_key_id.clone()],
    });
    state.fields[parent_index.clone()]
        .xml_attribute_info
        .xml_attributes_map
        .insert(map_key_obj, attr_new_info.clone());
}

fn abort_attributes_case_attr(
    state: &mut State,
    xml_key: &String,
    xml_attr_info: XmlAttributeArrayinfo,
) {
    let parent_index = state.fields.len() - 2;
    let map_key_obj = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonObjectNestinState,
    };
    let attr_new_info = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
        unique_key_ids: xml_attr_info.unique_key_ids.clone(),
    });
    state.fields[parent_index.clone()]
        .xml_attribute_info
        .xml_attributes_map
        .insert(map_key_obj, attr_new_info.clone());
}

pub fn abort_attributes(state: &mut State, xml_key: &String) {
    if state.fields.len() < 2 {
        return;
    }
    match get_attributes_type(state, xml_key) {
        Some(XmlAttributesType::ObjectAttributes(xml_attr_info)) => {
            abort_attributes_case_obj(state, xml_key, xml_attr_info)
        }
        Some(XmlAttributesType::ArrayTypeAttributes(xml_attr_info)) => {
            abort_attributes_case_attr(state, xml_key, xml_attr_info)
        }
        _ => (),
    }
}
