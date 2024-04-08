use crate::json_to_xml::{state::State, xml_attributes::models::XmlAttributesType};

use super::{
    get_attributes_type_mut,
    models::{XmlAttribute, XmlAttributeArrayinfo, XmlAttributeObjectInfo, XmlAttributesBasicInfo},
};

fn update_xmlattribute_key_arr(
    xml_atrribute_key: &String,
    array_type_info: &mut XmlAttributeArrayinfo,
) {
    let new_attr = XmlAttribute {
        xml_attribute_value: String::new(),
        xml_atrribute_key: xml_atrribute_key.clone(),
    };
    if array_type_info.attributes.len() == 0
        || array_type_info.attributes.len() - 1 != array_type_info.current_item_index
    {
        array_type_info.attributes.push(vec![new_attr])
    } else {
        let last_indx = array_type_info.attributes.len() - 1;
        array_type_info.attributes[last_indx].push(new_attr);
    }
}

fn update_xmlattribute_key_obj(
    xml_atrribute_key: &String,
    object_type_info: &mut XmlAttributeObjectInfo,
) {
    object_type_info.attributes.push(XmlAttribute {
        xml_attribute_value: String::new(),
        xml_atrribute_key: xml_atrribute_key.clone(),
    });
}

pub fn update_xml_attribute_key_found_entry(
    xml_atrribute_key: &String,
    xml_attributes_info: &mut XmlAttributesType,
) {
    match xml_attributes_info {
        XmlAttributesType::ArrayTypeAttributes(array_type_info) => {
            update_xmlattribute_key_arr(xml_atrribute_key, array_type_info)
        }
        XmlAttributesType::ObjectAttributes(object_type_info) => {
            update_xmlattribute_key_obj(xml_atrribute_key, object_type_info)
        }
        XmlAttributesType::NoAttribute(_) => (),
    }
}

pub fn update_xml_attribute_key(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    xml_atrribute_key: &String,
) {
    match get_attributes_type_mut(state, &basic_info.clone()) {
        Some(xml_attributes_info) => {
            update_xml_attribute_key_found_entry(xml_atrribute_key, xml_attributes_info);
        }
        None => (),
    }
}

fn update_xmlattribute_val_arr(
    xml_atrribute_value: &String,
    array_type_info: &mut XmlAttributeArrayinfo,
) {
    if array_type_info.attributes.len() < 1 {
        return;
    }

    if array_type_info.attributes.len() - 1 == array_type_info.current_item_index {
        let last_indx_vec_of_vec = array_type_info.attributes.len() - 1;
        if let true = array_type_info.attributes[last_indx_vec_of_vec].len() > 0 {
            let last_indx_vec = array_type_info.attributes[last_indx_vec_of_vec].len() - 1;
            array_type_info.attributes[last_indx_vec_of_vec][last_indx_vec].xml_attribute_value =
                xml_atrribute_value.clone();
        }
    }
}

fn update_xmlattribute_val_obj(
    xml_atrribute_value: &String,
    object_type_info: &mut XmlAttributeObjectInfo,
) {
    match object_type_info.attributes.len() > 0 {
        true => {
            let last_indx = object_type_info.attributes.len() - 1;
            object_type_info.attributes[last_indx].xml_attribute_value =
                xml_atrribute_value.clone();
        }
        _ => (),
    }
}

pub fn update_xml_attribute_val_found_entry(
    xml_atrribute_value: &String,
    xml_attributes_info: &mut XmlAttributesType,
) {
    match xml_attributes_info {
        XmlAttributesType::ArrayTypeAttributes(array_type_info) => {
            update_xmlattribute_val_arr(xml_atrribute_value, array_type_info)
        }
        XmlAttributesType::ObjectAttributes(object_type_info) => {
            update_xmlattribute_val_obj(xml_atrribute_value, object_type_info)
        }
        XmlAttributesType::NoAttribute(_) => (),
    }
}

pub fn update_xml_attribute_value(
    state: &mut State,
    basic_info: &XmlAttributesBasicInfo,
    xml_atrribute_value: &String,
) {
    match get_attributes_type_mut(state, &basic_info) {
        Some(xml_attributes_info) => {
            update_xml_attribute_val_found_entry(xml_atrribute_value, xml_attributes_info);
        }
        None => (),
    }
}

fn update_xmlattribute_arr_index(array_type_info: &mut XmlAttributeArrayinfo) {
    array_type_info.current_item_index += 1;
}
pub fn update_xmlattribute_found_entry_arr_index(xml_attributes_info: &mut XmlAttributesType) {
    match xml_attributes_info {
        XmlAttributesType::ArrayTypeAttributes(array_type_info) => {
            update_xmlattribute_arr_index(array_type_info)
        }
        _ => (),
    }
}

pub fn update_xml_attribute_arr_index(state: &mut State, basic_info: &XmlAttributesBasicInfo) {
    match get_attributes_type_mut(state, &basic_info) {
        Some(xml_attributes_info) => {
            update_xmlattribute_found_entry_arr_index(xml_attributes_info);
        }
        None => (),
    }
}
