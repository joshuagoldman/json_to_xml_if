use uuid::Uuid;

use crate::json_to_if::{
    models::NestingState,
    state::State,
    xml_attributes::{
        self,
        models::{
            XmlAttributeArrayinfo, XmlAttributeObjectInfo, XmlAttributesMapKey, XmlAttributesType,
        },
    },
};

use super::models::{XmlAttributeNoAttributeInfo, XmlAttributesUniqIds};

pub fn abort_attributes(state: &mut State, xml_key: &String) {
    let last_indx = state.fields.len() - 1;
    match get_attributes_type(state, xml_key) {
        Some(XmlAttributesType::ObjectAttributes(xml_attr_info)) => {
            let map_key_obj = XmlAttributesMapKey {
                attribute_base_name: xml_key.clone(),
                attribute_type: NestingState::JsonObjectNestinState,
            };
            let attr_new_info = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
                unique_key_ids: vec![xml_attr_info.unique_key_id.clone()],
            });
            state.fields[last_indx.clone()]
                .clone()
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_obj, attr_new_info.clone());
        }
        Some(XmlAttributesType::ArrayTypeAttributes(xml_attr_info)) => {
            let map_key_obj = XmlAttributesMapKey {
                attribute_base_name: xml_key.clone(),
                attribute_type: NestingState::JsonObjectNestinState,
            };
            let attr_new_info = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
                unique_key_ids: xml_attr_info.unique_key_ids.clone(),
            });
            state.fields[last_indx.clone()]
                .clone()
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_obj, attr_new_info.clone());
        }
        _ => (),
    }
}
pub fn get_attributes_type<'a>(state: &mut State, xml_key: &String) -> Option<XmlAttributesType> {
    let last_indx = state.fields.len() - 1;
    let last_field = state.fields[last_indx.clone()].clone();
    let nesting_state = last_field.nesting_state.clone();
    let map_key = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: nesting_state,
    };
    state.fields[last_indx.clone()]
        .xml_attribute_info
        .xml_attributes_map
        .get(&map_key)
        .cloned()
}

pub fn get_attributes_type_mut<'a>(
    state: &'a mut State,
    xml_key: &String,
) -> Option<&'a mut XmlAttributesType> {
    let last_indx = state.fields.len() - 1;
    let last_field = state.fields[last_indx.clone()].clone();
    let nesting_state = last_field.nesting_state.clone();
    let map_key = XmlAttributesMapKey {
        attribute_base_name: xml_key.clone(),
        attribute_type: nesting_state,
    };
    state.fields[last_indx.clone()]
        .xml_attribute_info
        .xml_attributes_map
        .get_mut(&map_key)
}

pub fn get_attributes_object_id(
    state: &mut State,
    xml_key: &String,
    nesting_state: NestingState,
) -> Option<XmlAttributesUniqIds> {
    if !xml_key.to_uppercase().ends_with("_ATTRIBUTES") {
        return None;
    }
    let last_indx = state.fields.len() - 1;
    let unique_id = Uuid::new_v4().to_string();
    let object_id = Uuid::new_v4().to_string();
    match (get_attributes_type(state, xml_key), nesting_state) {
        (
            Some(XmlAttributesType::NoAttribute(xml_attr_info)),
            NestingState::JsonArrayNestingState,
        ) => {
            let object_id = Uuid::new_v4().to_string();
            let map_key_obj = XmlAttributesMapKey {
                attribute_base_name: xml_key.clone(),
                attribute_type: NestingState::JsonArrayNestingState,
            };
            let mut new_xml_attr_info = xml_attr_info.clone();
            new_xml_attr_info.unique_key_ids.push(unique_id.clone());
            let array_attributes_info =
                XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
                    unique_key_ids: new_xml_attr_info.unique_key_ids,
                    object_id: object_id.clone(),
                    attributes: Vec::new(),
                });
            state.fields[last_indx.clone()]
                .clone()
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_obj, array_attributes_info.clone());

            Some(XmlAttributesUniqIds {
                attr_id: unique_id.clone(),
                attr_object_id: object_id.clone(),
            })
        }
        (
            Some(XmlAttributesType::NoAttribute(xml_attr_info)),
            NestingState::JsonObjectNestinState,
        ) => {
            let object_id = Uuid::new_v4().to_string();
            let map_key_obj = XmlAttributesMapKey {
                attribute_base_name: xml_key.clone(),
                attribute_type: NestingState::JsonObjectNestinState,
            };
            let object_attributes_info =
                XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
                    unique_key_id: xml_attr_info.unique_key_ids[0].clone(),
                    object_id: object_id.clone(),
                    attributes: Vec::new(),
                });
            state.fields[last_indx.clone()]
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_obj, object_attributes_info.clone());

            Some(XmlAttributesUniqIds {
                attr_id: xml_attr_info.clone().unique_key_ids[0].clone(),
                attr_object_id: object_id.clone(),
            })
        }
        (None, NestingState::JsonObjectNestinState) => {
            let map_key_obj = XmlAttributesMapKey {
                attribute_base_name: xml_key.clone(),
                attribute_type: NestingState::JsonObjectNestinState,
            };
            let object_attributes_info =
                XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
                    unique_key_id: unique_id.clone(),
                    object_id: object_id.clone(),
                    attributes: Vec::new(),
                });
            state.fields[last_indx.clone()]
                .clone()
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_obj, object_attributes_info.clone());
            Some(XmlAttributesUniqIds {
                attr_id: unique_id.clone(),
                attr_object_id: object_id.clone(),
            })
        }
        (None, NestingState::JsonArrayNestingState) => {
            let map_key_obj = XmlAttributesMapKey {
                attribute_base_name: xml_key.clone(),
                attribute_type: NestingState::JsonArrayNestingState,
            };
            let new_id_vec = vec![unique_id.to_string()];
            let array_attributes_info =
                XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
                    unique_key_ids: new_id_vec,
                    object_id: object_id.clone(),
                    attributes: Vec::new(),
                });
            state.fields[last_indx.clone()]
                .clone()
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_obj, array_attributes_info.clone());
            Some(XmlAttributesUniqIds {
                attr_id: unique_id.clone(),
                attr_object_id: object_id.clone(),
            })
        }
        _ => None,
    }
}

pub fn get_attributes_mark(state: &mut State, xml_key: &String) -> String {
    let mut unique_id = String::new();
    let last_indx = state.fields.len() - 1;
    match get_attributes_type_mut(state, xml_key) {
        Some(xml_attributes_info) => match xml_attributes_info {
            xml_attributes::models::XmlAttributesType::ArrayTypeAttributes(xml_attributes_info) => {
                let unique_id = Uuid::new_v4().to_string();
                xml_attributes_info
                    .clone()
                    .unique_key_ids
                    .push(unique_id.clone());
            }
            xml_attributes::models::XmlAttributesType::ObjectAttributes(xml_attributes_info) => {
                let unique_id = Uuid::new_v4().to_string();
                xml_attributes_info.unique_key_id = unique_id;
            }
            xml_attributes::models::XmlAttributesType::NoAttribute(key_infos) => {
                unique_id = Uuid::new_v4().to_string();
                key_infos.unique_key_ids.push(unique_id.clone());
            }
        },
        None => {
            unique_id = Uuid::new_v4().to_string();
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
            state.fields[last_indx.clone()]
                .clone()
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_obj, new_key_infos.clone());
            state.fields[last_indx.clone()]
                .clone()
                .xml_attribute_info
                .xml_attributes_map
                .insert(map_key_arr, new_key_infos);
        }
    }
    unique_id
}
