use crate::json_to_if::{
    state::State,
    xml_attributes::models::{XmlAttributesMapKey, XmlAttributesType},
};

use super::models::{XmlAttribute, XmlAttributeArrayinfo, XmlAttributeObjectInfo};

fn update_xmlattribute_key_arr(
    xml_atrribute_key: &String,
    array_type_info: &mut XmlAttributeArrayinfo,
) {
    let mut new_attr_vec = array_type_info.attributes.last().unwrap().clone();

    new_attr_vec.push(XmlAttribute {
        xml_attribute_value: String::new(),
        xml_atrribute_key: xml_atrribute_key.clone(),
    });

    array_type_info.attributes.pop();
    array_type_info.attributes.push(new_attr_vec.clone());
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

pub fn update_xml_attribute_key(state: &mut State, xml_atrribute_key: &String) {
    let last_index = state.fields.len() - 1;
    let nesting_state = state.fields[last_index.clone()].nesting_state.clone();
    let key = if let Some(some_key) = state.fields[last_index.clone()].key.clone() {
        some_key
    } else {
        String::new()
    };
    let map_key = XmlAttributesMapKey {
        attribute_type: nesting_state,
        attribute_base_name: key,
    };
    match state.fields[last_index.clone()]
        .xml_attributes_map
        .get_mut(&map_key)
    {
        Some(xml_attributes_info) => {
            update_xml_attribute_key_found_entry(xml_atrribute_key, xml_attributes_info)
        }
        None => (),
    }
}

fn update_xmlattribute_val_arr(
    xml_atrribute_value: &String,
    array_type_info: &mut XmlAttributeArrayinfo,
) {
    let mut new_attr_vec = array_type_info.attributes.last().unwrap().clone();

    let mut last_attr_info = new_attr_vec.last().unwrap().clone();
    last_attr_info.xml_attribute_value = xml_atrribute_value.clone();

    new_attr_vec.pop();
    new_attr_vec.push(last_attr_info.clone());

    array_type_info.attributes.pop();
    array_type_info.attributes.push(new_attr_vec.clone());
}

fn update_xmlattribute_val_obj(
    xml_atrribute_value: &String,
    object_type_info: &mut XmlAttributeObjectInfo,
) {
    let mut last_attribute = object_type_info.attributes.last().unwrap().clone();
    last_attribute.xml_attribute_value = xml_atrribute_value.clone();

    object_type_info.attributes.pop();
    object_type_info.attributes.push(last_attribute.clone());
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

pub fn update_xml_attribute_value(state: &mut State, xml_atrribute_value: &String) {
    let last_index = state.fields.len() - 1;
    let nesting_state = state.fields[last_index.clone()].nesting_state.clone();

    let key: String;
    if let Some(some_key) = state.fields[last_index.clone()].key.clone() {
        key = some_key;
    } else {
        return;
    }

    let map_key = XmlAttributesMapKey {
        attribute_type: nesting_state,
        attribute_base_name: key,
    };
    match state.fields[last_index.clone()]
        .xml_attributes_map
        .get_mut(&map_key)
    {
        Some(xml_attributes_info) => {
            update_xml_attribute_val_found_entry(xml_atrribute_value, xml_attributes_info)
        }
        None => (),
    }
}
