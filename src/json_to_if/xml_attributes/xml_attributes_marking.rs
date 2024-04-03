use uuid::Uuid;

use crate::json_to_if::{
    state::State,
    xml_attributes::{
        self,
        models::{XmlAttributesMapKey, XmlAttributesType},
    },
};

use super::models::{XmlAttributeArrayinfo, XmlAttributeNoAttributeInfo, XmlAttributeObjectInfo};

fn get_attr_mark_case_attr_reg_arr(
    xml_attributes_info: &mut XmlAttributeArrayinfo,
) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    xml_attributes_info.unique_key_ids.push(unique_id.clone());
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
    xml_attributes_info.unique_key_ids.push(unique_id.clone());
    Some(unique_id)
}

fn get_attr_mark_case_not_reg(
    state: &mut State,
    map_key: &XmlAttributesMapKey,
    attr_id: &String,
) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    let new_id_vec = vec![unique_id.to_string()];
    let new_key_infos = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
        unique_key_ids: new_id_vec,
    });

    match state.xml_attributes_map.get_mut(attr_id) {
        Some(attr_map) => {
            attr_map.insert(map_key.clone(), new_key_infos);
            Some(unique_id)
        }
        None => None,
    }
}

pub fn get_attributes_mark(state: &mut State, key: &String) -> Option<String> {
    if state.fields.len() < 2 {
        return None;
    }
    let parent_index = state.fields.len() - 2;
    let last_index = state.fields.len() - 2;
    let parent_field = state.fields[parent_index.clone()].clone();
    let last_field = state.fields[last_index.clone()].clone();
    let map_key = XmlAttributesMapKey {
        attribute_base_name: key.clone(),
        attribute_type: last_field.nesting_state,
    };

    match state
        .xml_attributes_map
        .get_mut(&parent_field.xml_attributes_map_id)
    {
        Some(attr_map) => match attr_map.get_mut(&map_key) {
            Some(xml_attributes_type) => match xml_attributes_type {
                xml_attributes::models::XmlAttributesType::ArrayTypeAttributes(
                    xml_attributes_info,
                ) => get_attr_mark_case_attr_reg_arr(xml_attributes_info),
                xml_attributes::models::XmlAttributesType::ObjectAttributes(
                    xml_attributes_info,
                ) => get_attr_mark_case_attr_reg_obj(xml_attributes_info),
                xml_attributes::models::XmlAttributesType::NoAttribute(key_infos) => {
                    get_attr_mark_case_only_none_attr_reg(key_infos)
                }
            },
            None => get_attr_mark_case_not_reg(state, &map_key, &last_field.xml_attributes_map_id),
        },
        None => None,
    }
}
