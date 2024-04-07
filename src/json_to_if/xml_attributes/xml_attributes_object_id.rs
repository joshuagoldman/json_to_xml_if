use uuid::Uuid;

use crate::json_to_if::{
    models::NestingState,
    state::State,
    xml_attributes::models::{XmlAttributeArrayinfo, XmlAttributeObjectInfo},
};

use super::{
    get_attributes_type, get_attributes_type_mut,
    models::{
        AttributeObjectPairs, XmlAttributeNoAttributeInfo, XmlAttributesBasicInfo,
        XmlAttributesType,
    },
};

fn get_atrributes_object_id_case_already_arr_attr(
    xml_attr_info: &XmlAttributeArrayinfo,
) -> Option<String> {
    Some(xml_attr_info.object_id.clone())
}

fn get_atrributes_object_id_case_no_attr_arr(
    xml_attr_type: &mut XmlAttributesType,
    xml_attr_info: &XmlAttributeNoAttributeInfo,
) -> Option<String> {
    let object_id = Uuid::new_v4().to_string();
    let array_attributes_info = XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
        unique_key_ids: xml_attr_info.unique_key_ids.clone(),
        object_id: object_id.clone(),
        attributes: Vec::new(),
        current_item_index: 0,
        object_pairs_info: AttributeObjectPairs {
            has_attribute_obj: true,
            has_none_attribute_obj: true,
        },
    });

    *xml_attr_type = array_attributes_info;

    Some(object_id)
}

fn get_atrributes_object_id_case_no_attr_obj(
    xml_attr_type: &mut XmlAttributesType,
    xml_attr_info: &XmlAttributeNoAttributeInfo,
) -> Option<String> {
    let object_id = Uuid::new_v4().to_string();
    let object_attributes_info = XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
        unique_key_ids: xml_attr_info.unique_key_ids.clone(),
        object_id: object_id.clone(),
        attributes: Vec::new(),
        object_pairs_info: AttributeObjectPairs {
            has_attribute_obj: true,
            has_none_attribute_obj: true,
        },
    });

    *xml_attr_type = object_attributes_info;

    Some(object_id)
}

fn get_atrributes_object_id_case_not_in_dict_arr(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) -> Option<String> {
    let object_id = Uuid::new_v4().to_string();

    let array_attributes_info = XmlAttributesType::ArrayTypeAttributes(XmlAttributeArrayinfo {
        unique_key_ids: vec![],
        object_id: object_id.clone(),
        attributes: Vec::new(),
        current_item_index: 0,
        object_pairs_info: AttributeObjectPairs {
            has_attribute_obj: true,
            has_none_attribute_obj: false,
        },
    });

    match state.xml_attributes_map.get_mut(&basic_info.attr_id) {
        Some(attr_map) => {
            attr_map.insert(basic_info.current_key.clone(), array_attributes_info);
        }
        None => (),
    }
    Some(object_id.clone())
}

fn get_atrributes_object_id_case_not_in_dict_obj(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) -> Option<String> {
    let object_id = Uuid::new_v4().to_string();
    let unique_id = Uuid::new_v4().to_string();
    let obj_attributes_info = XmlAttributesType::ObjectAttributes(XmlAttributeObjectInfo {
        unique_key_ids: vec![unique_id.clone()],
        object_id: object_id.clone(),
        attributes: Vec::new(),
        object_pairs_info: AttributeObjectPairs {
            has_attribute_obj: true,
            has_none_attribute_obj: false,
        },
    });

    match state.xml_attributes_map.get_mut(&basic_info.attr_id) {
        Some(attr_map) => {
            attr_map.insert(basic_info.current_key.clone(), obj_attributes_info);
        }
        None => (),
    }
    Some(object_id)
}

pub fn get_attributes_object_id(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
) -> Option<String> {
    match get_attributes_type_mut(state, basic_info) {
        Some(xml_attr_type) => match (
            xml_attr_type.clone(),
            basic_info.current_key.attribute_type.clone(),
        ) {
            (
                XmlAttributesType::NoAttribute(xml_attr_info),
                NestingState::JsonArrayNestingState,
            ) => get_atrributes_object_id_case_no_attr_arr(xml_attr_type, &xml_attr_info),
            (
                XmlAttributesType::NoAttribute(xml_attr_info),
                NestingState::JsonObjectNestinState,
            ) => get_atrributes_object_id_case_no_attr_obj(xml_attr_type, &xml_attr_info),
            (
                XmlAttributesType::ArrayTypeAttributes(xml_attr_info),
                NestingState::JsonArrayNestingState,
            ) => get_atrributes_object_id_case_already_arr_attr(&xml_attr_info),
            _ => None,
        },
        _ => match basic_info.current_key.attribute_type {
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
