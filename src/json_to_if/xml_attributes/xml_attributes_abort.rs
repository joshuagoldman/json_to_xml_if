use crate::json_to_if::{state::State, xml_attributes::models::XmlAttributesType};

use super::{
    get_attributes_type_mut,
    models::{
        XmlAttributeArrayinfo, XmlAttributeNoAttributeInfo, XmlAttributeObjectInfo,
        XmlAttributesBasicInfo,
    },
};

fn abort_attributes_case_obj(
    xml_attr_type: &mut XmlAttributesType,
    xml_attr_info: &XmlAttributeObjectInfo,
) -> String {
    let obj_new_info = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
        unique_key_ids: xml_attr_info.unique_key_ids.clone(),
    });

    *xml_attr_type = obj_new_info;

    xml_attr_info.object_id.clone()
}

fn abort_attributes_case_attr(
    xml_attr_type: &mut XmlAttributesType,
    xml_attr_info: &XmlAttributeArrayinfo,
) -> String {
    let attr_new_info = XmlAttributesType::NoAttribute(XmlAttributeNoAttributeInfo {
        unique_key_ids: xml_attr_info.unique_key_ids.clone(),
    });

    *xml_attr_type = attr_new_info;

    xml_attr_info.object_id.clone()
}

pub fn abort_attributes(state: &mut State, basic_info: &XmlAttributesBasicInfo) {
    let mut object_id_to_remove = String::new();
    match get_attributes_type_mut(state, basic_info) {
        Some(xml_attr_type) => match xml_attr_type.clone() {
            XmlAttributesType::ObjectAttributes(xml_attr_info) => {
                object_id_to_remove = abort_attributes_case_obj(xml_attr_type, &xml_attr_info);
            }
            XmlAttributesType::ArrayTypeAttributes(xml_attr_info) => {
                object_id_to_remove = abort_attributes_case_attr(xml_attr_type, &xml_attr_info);
            }
            _ => (),
        },
        None => (),
    }
    if !object_id_to_remove.is_empty() {
        state.curr_xml = state.curr_xml.replace(object_id_to_remove.as_str(), "");
    }
}
