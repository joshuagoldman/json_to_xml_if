use uuid::Uuid;

use crate::json_to_if::{
    models::NestingState,
    state::State,
    xml_attributes::models::{XmlAttributeArrayinfo, XmlAttributeObjectInfo},
};

use super::{
    get_attributes_type, get_attributes_type_mut,
    models::{
        XmlAttributeNoAttributeInfo, XmlAttributesBasicInfo, XmlAttributesType,
        XmlAttributesUniqIds,
    },
};

fn get_atrributes_object_id_case_no_attr_arr(
    xml_attr_type: &mut XmlAttributesType,
    xml_attr_info: &XmlAttributeNoAttributeInfo,
) -> Option<XmlAttributesUniqIds> {
    let unique_id = Uuid::new_v4().to_string();
    let object_id = Uuid::new_v4().to_string();
    let mut new_xml_attr_info = xml_attr_info.clone();
    new_xml_attr_info.unique_key_ids.push(unique_id.clone());
    let array_attributes_info = XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
        unique_key_ids: new_xml_attr_info.unique_key_ids,
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });

    *xml_attr_type = array_attributes_info;

    Some(XmlAttributesUniqIds {
        attr_id: unique_id.clone(),
        attr_object_id: object_id.clone(),
    })
}

fn get_atrributes_object_id_case_no_attr_obj(
    xml_attr_type: &mut XmlAttributesType,
    xml_attr_info: &XmlAttributeNoAttributeInfo,
) -> Option<XmlAttributesUniqIds> {
    let object_id = Uuid::new_v4().to_string();
    let object_attributes_info = XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
        unique_key_id: xml_attr_info.unique_key_ids[0].clone(),
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });

    *xml_attr_type = object_attributes_info;

    Some(XmlAttributesUniqIds {
        attr_id: xml_attr_info.clone().unique_key_ids[0].clone(),
        attr_object_id: object_id.clone(),
    })
}

fn get_atrributes_object_id_case_not_in_dict_arr(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) -> Option<XmlAttributesUniqIds> {
    let object_id = Uuid::new_v4().to_string();
    let unique_id = Uuid::new_v4().to_string();
    let unique_ids_vec = vec![unique_id.to_string()];

    let array_attributes_info = XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
        unique_key_ids: unique_ids_vec,
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });

    match state.xml_attributes_map.get_mut(&basic_info.attr_id) {
        Some(attr_map) => {
            attr_map.insert(basic_info.current_key.clone(), array_attributes_info);
        }
        None => (),
    }
    Some(XmlAttributesUniqIds {
        attr_id: unique_id,
        attr_object_id: object_id,
    })
}

fn get_atrributes_object_id_case_not_in_dict_obj(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) -> Option<XmlAttributesUniqIds> {
    let object_id = Uuid::new_v4().to_string();
    let unique_id = Uuid::new_v4().to_string();
    let obj_attributes_info = XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
        unique_key_id: unique_id.to_string(),
        object_id: object_id.clone(),
        attributes: Vec::new(),
    });

    match state.xml_attributes_map.get_mut(&basic_info.attr_id) {
        Some(attr_map) => {
            attr_map.insert(basic_info.current_key.clone(), obj_attributes_info);
        }
        None => (),
    }
    Some(XmlAttributesUniqIds {
        attr_id: unique_id.clone(),
        attr_object_id: object_id.clone(),
    })
}

pub fn get_attributes_object_id(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) -> Option<XmlAttributesUniqIds> {
    let last_indx = state.fields.len() - 1;
    let nesting_state = state.fields[last_indx].nesting_state.clone();

    match get_attributes_type_mut(state, basic_info) {
        Some(xml_attr_type) => match (xml_attr_type.clone(), nesting_state) {
            (
                XmlAttributesType::NoAttribute(xml_attr_info),
                NestingState::JsonArrayNestingState,
            ) => get_atrributes_object_id_case_no_attr_arr(xml_attr_type, &xml_attr_info),
            (
                XmlAttributesType::NoAttribute(xml_attr_info),
                NestingState::JsonObjectNestinState,
            ) => get_atrributes_object_id_case_no_attr_obj(xml_attr_type, &xml_attr_info),
            _ => None,
        },
        _ => match nesting_state {
            NestingState::JsonObjectNestinState => {
                get_atrributes_object_id_case_not_in_dict_obj(state, basic_info)
            }
            NestingState::JsonArrayNestingState => {
                get_atrributes_object_id_case_not_in_dict_arr(state, basic_info)
            }
        },
    }
}

pub fn get_attributes_object_id_for_closing_tag(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) -> Option<String> {
    match get_attributes_type(state, basic_info) {
        Some(xml_attr_type) => match xml_attr_type {
            XmlAttributesType::ArrayTypeAttributes(xml_attr_info) => {
                Some(xml_attr_info.object_id.clone())
            }
            XmlAttributesType::ObjectAttributes(xml_attr_info) => {
                Some(xml_attr_info.object_id.clone())
            }
            XmlAttributesType::NoAttribute(_) => None,
        },
        None => None,
    }
}
