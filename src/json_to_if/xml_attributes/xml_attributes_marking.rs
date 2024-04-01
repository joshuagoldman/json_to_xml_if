use uuid::Uuid;

use crate::json_to_if::{
    models::NestingState,
    state::State,
    xml_attributes::{
        self,
        models::{XmlAttributesMapKey, XmlAttributesType},
    },
};

use super::{
    get_attributes_type_mut,
    models::{XmlAttributeArrayinfo, XmlAttributeNoAttributeInfo, XmlAttributeObjectInfo},
};

fn get_attr_mark_case_attr_reg_arr(
    xml_attributes_info: &mut XmlAttributeArrayinfo,
) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    xml_attributes_info
        .clone()
        .unique_key_ids
        .push(unique_id.clone());
    Some(unique_id)
}

fn get_attr_mark_case_attr_reg_obj(
    xml_attributes_info: &mut XmlAttributeObjectInfo,
) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    xml_attributes_info.unique_key_id = unique_id.clone();
    Some(unique_id)
}

fn get_attr_mark_case_only_none_attr_reg(
    xml_attributes_info: &mut XmlAttributeNoAttributeInfo,
) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    xml_attributes_info
        .clone()
        .unique_key_ids
        .push(unique_id.clone());
    Some(unique_id)
}

fn get_attr_mark_case_not_reg(state: &mut State, xml_key: &String) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    let parent_index = state.fields.len() - 2;
    let map_key_obj = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonObjectNestinState,
    };
    let map_key_arr = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonObjectNestinState,
    };
    let new_id_vec = vec![unique_id.to_string()];
    let new_key_infos = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
        unique_key_ids: new_id_vec,
    });
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .insert(map_key_obj, new_key_infos.clone());
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .insert(map_key_arr, new_key_infos);

    Some(unique_id)
}

pub fn get_attributes_mark(state: &mut State, xml_key: &String) -> Option<String> {
    if state.fields.len() < 2 {
        return None;
    }
    match get_attributes_type_mut(state, xml_key) {
        Some(xml_attributes_info) => match xml_attributes_info {
            xml_attributes::models::XmlAttributesType::ArrayTypeAttributes(xml_attributes_info) => {
                get_attr_mark_case_attr_reg_arr(xml_attributes_info)
            }
            xml_attributes::models::XmlAttributesType::ObjectAttributes(xml_attributes_info) => {
                get_attr_mark_case_attr_reg_obj(xml_attributes_info)
            }
            xml_attributes::models::XmlAttributesType::NoAttribute(key_infos) => {
                get_attr_mark_case_only_none_attr_reg(key_infos)
            }
        },
        None => get_attr_mark_case_not_reg(state, xml_key),
    }
}
