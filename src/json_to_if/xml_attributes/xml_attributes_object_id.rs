use uuid::Uuid;

use crate::json_to_if::{
    models::NestingState,
    state::State,
    xml_attributes::models::{
        XmlAttributeArrayinfo, XmlAttributeObjectInfo, XmlAttributesMapKey, XmlAttributesType,
    },
};

use super::{
    get_attributes_type,
    models::{XmlAttributeNoAttributeInfo, XmlAttributesUniqIds},
};

fn get_atrributes_object_id_case_no_attr_arr(
    state: &mut State,
    xml_key: &String,
    xml_attr_info: XmlAttributeNoAttributeInfo,
) -> Option<XmlAttributesUniqIds> {
    let parent_index = state.fields.len() - 2;
    let unique_id = Uuid::new_v4().to_string();
    let object_id = Uuid::new_v4().to_string();
    let map_key_obj = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonArrayNestingState,
    };
    let mut new_xml_attr_info = xml_attr_info.clone();
    new_xml_attr_info.unique_key_ids.push(unique_id.clone());
    let array_attributes_info = XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
        unique_key_ids: new_xml_attr_info.unique_key_ids,
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .insert(map_key_obj, array_attributes_info.clone());

    Some(XmlAttributesUniqIds {
        attr_id: unique_id.clone(),
        attr_object_id: object_id.clone(),
    })
}

fn get_atrributes_object_id_case_no_attr_obj(
    state: &mut State,
    xml_key: &String,
    xml_attr_info: XmlAttributeNoAttributeInfo,
) -> Option<XmlAttributesUniqIds> {
    let parent_index = state.fields.len() - 2;
    let object_id = Uuid::new_v4().to_string();
    let map_key_obj = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonObjectNestinState,
    };
    let object_attributes_info = XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
        unique_key_id: xml_attr_info.unique_key_ids[0].clone(),
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .insert(map_key_obj, object_attributes_info.clone());

    Some(XmlAttributesUniqIds {
        attr_id: xml_attr_info.clone().unique_key_ids[0].clone(),
        attr_object_id: object_id.clone(),
    })
}

fn get_atrributes_object_id_case_not_in_dict_arr(
    state: &mut State,
    xml_key: &String,
) -> Option<XmlAttributesUniqIds> {
    let parent_index = state.fields.len() - 2;
    let object_id = Uuid::new_v4().to_string();
    let unique_id = Uuid::new_v4().to_string();
    let map_key_obj = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonObjectNestinState,
    };
    let object_attributes_info = XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
        unique_key_id: unique_id.clone(),
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .insert(map_key_obj, object_attributes_info.clone());
    Some(XmlAttributesUniqIds {
        attr_id: unique_id.clone(),
        attr_object_id: object_id.clone(),
    })
}

fn get_atrributes_object_id_case_not_in_dict_obj(
    state: &mut State,
    xml_key: &String,
) -> Option<XmlAttributesUniqIds> {
    let parent_index = state.fields.len() - 2;
    let object_id = Uuid::new_v4().to_string();
    let unique_id = Uuid::new_v4().to_string();
    let map_key_obj = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: NestingState::JsonArrayNestingState,
    };
    let new_id_vec = vec![unique_id.to_string()];
    let array_attributes_info = XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
        unique_key_ids: new_id_vec,
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });
    state.fields[parent_index.clone()]
        .xml_attributes_map
        .insert(map_key_obj, array_attributes_info.clone());
    Some(XmlAttributesUniqIds {
        attr_id: unique_id.clone(),
        attr_object_id: object_id.clone(),
    })
}

pub fn get_attributes_object_id(
    state: &mut State,
    xml_key: &String,
) -> Option<XmlAttributesUniqIds> {
    let last_indx = state.fields.len() - 1;
    let nesting_state = state.fields[last_indx].nesting_state.clone();
    match (get_attributes_type(state, xml_key), nesting_state) {
        (
            Some(XmlAttributesType::NoAttribute(xml_attr_info)),
            NestingState::JsonArrayNestingState,
        ) => get_atrributes_object_id_case_no_attr_arr(state, xml_key, xml_attr_info),
        (
            Some(XmlAttributesType::NoAttribute(xml_attr_info)),
            NestingState::JsonObjectNestinState,
        ) => get_atrributes_object_id_case_no_attr_obj(state, xml_key, xml_attr_info),
        (None, NestingState::JsonObjectNestinState) => {
            get_atrributes_object_id_case_not_in_dict_obj(state, xml_key)
        }
        (None, NestingState::JsonArrayNestingState) => {
            get_atrributes_object_id_case_not_in_dict_arr(state, xml_key)
        }
        _ => None,
    }
}

pub fn get_attributes_object_id_for_closing_tag(
    state: &mut State,
    xml_key: &String,
) -> Option<String> {
    let last_indx = state.fields.len() - 1;
    let nesting_state = state.fields[last_indx].nesting_state.clone();
    match get_attributes_type(state, xml_key) {
        Some(xml_attr_type) => match xml_attr_type {
            XmlAttributesType::ArrayTypeAttributes(xml_attr_info) => Some(xml_attr_info.object_id),
            XmlAttributesType::ObjectAttributes(xml_attr_info) => Some(xml_attr_info.object_id),
            XmlAttributesType::NoAttribute(_) => None,
        },
        None => None,
    }
}
