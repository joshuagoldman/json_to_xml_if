use uuid::Uuid;

use crate::json_to_if::{
    models::{FieldPositionNumForMap, NestingState},
    state::State,
    xml_attributes::{
        self,
        models::{XmlAttributesMapKey, XmlAttributesType},
    },
};

use super::models::{
    AttributeObjectPairs, XmlAttributeArrayinfo, XmlAttributeNoAttributeInfo,
    XmlAttributeObjectInfo,
};

fn get_attr_mark_case_attr_reg_arr(
    xml_attributes_info: &mut XmlAttributeArrayinfo,
) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    xml_attributes_info.unique_key_ids.push(unique_id.clone());
    xml_attributes_info.object_pairs_info = AttributeObjectPairs {
        has_none_attribute_obj: true,
        has_attribute_obj: true,
    };
    Some(unique_id)
}

fn get_attr_mark_case_attr_reg_obj(
    xml_attributes_info: &mut XmlAttributeObjectInfo,
) -> Option<String> {
    let unique_id = Uuid::new_v4().to_string();
    xml_attributes_info.unique_key_ids.push(unique_id.clone());
    xml_attributes_info.object_pairs_info = AttributeObjectPairs {
        has_none_attribute_obj: true,
        has_attribute_obj: true,
    };
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
        object_id: None,
    });

    match state.xml_attributes_map.get_mut(attr_id) {
        Some(attr_map) => {
            attr_map.insert(map_key.clone(), new_key_infos);
            Some(unique_id)
        }
        None => None,
    }
}

pub fn get_attributes_mark(
    state: &mut State,
    key: &String,
    field_pos_with_rel_map: FieldPositionNumForMap,
) -> Option<String> {
    if state.fields.len() < field_pos_with_rel_map.xml_attr_map_num {
        return None;
    }
    let map_index = state.fields.len() - field_pos_with_rel_map.xml_attr_map_num;
    let map_field = state.fields[map_index.clone()].clone();
    let type_index = state.fields.len() - field_pos_with_rel_map.xml_attr_type_num;
    let type_field = state.fields[type_index.clone()].clone();

    let map_key = XmlAttributesMapKey {
        attribute_base_name: key.clone(),
        attribute_type: type_field.nesting_state,
    };

    match state
        .xml_attributes_map
        .get_mut(&map_field.xml_attributes_map_id)
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
            None => get_attr_mark_case_not_reg(state, &map_key, &map_field.xml_attributes_map_id),
        },
        None => None,
    }
}
