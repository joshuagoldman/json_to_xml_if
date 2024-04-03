use iter_tools::Itertools;

use crate::json_to_if::state::State;

use super::models::{XmlAttributeArrayinfo, XmlAttributeNoAttributeInfo, XmlAttributeObjectInfo};

pub fn check_end_xml_attributes_array_handling(
    state: &mut State,
    xml_attributes_array_info: XmlAttributeArrayinfo,
) {
    let xml_attibutes_vec_str = construct_xml_attributes_str_vec(&xml_attributes_array_info);
    remove_str_chunk_by_key(&mut state.curr_xml, &xml_attributes_array_info.object_id);
    for (i, id) in xml_attributes_array_info.unique_key_ids.iter().enumerate() {
        if xml_attibutes_vec_str.len() != 0 && xml_attibutes_vec_str.len() - 1 >= i {
            state.curr_xml = state
                .curr_xml
                .replace(id, xml_attibutes_vec_str[i].as_str());
        } else {
            state.curr_xml = state.curr_xml.replace(id, "")
        }
    }
}

pub fn check_end_xml_attributes_object_handling(
    state: &mut State,
    xml_attributes_object_info: XmlAttributeObjectInfo,
) {
    let xml_attibutes_vec_str = construct_xml_attributes_str(&xml_attributes_object_info);
    println!("{:#?}", xml_attibutes_vec_str);

    if xml_attibutes_vec_str.len() != 0 {
        state.curr_xml = state.curr_xml.replace(
            xml_attributes_object_info.unique_key_id.as_str(),
            xml_attibutes_vec_str.as_str(),
        );
        remove_str_chunk_by_key(&mut state.curr_xml, &xml_attributes_object_info.object_id);
    } else {
        state.curr_xml = state.curr_xml.replace(
            format!(" {}", xml_attributes_object_info.unique_key_id).as_str(),
            "",
        );
        state.curr_xml = state
            .curr_xml
            .replace(xml_attributes_object_info.object_id.as_str(), "");
    }
}

pub fn check_end_xml_no_attributes_handling(
    state: &mut State,
    keys_info: XmlAttributeNoAttributeInfo,
) {
    for (_, id) in keys_info.unique_key_ids.iter().enumerate() {
        state.curr_xml = state.curr_xml.replace(format!(" {}", id).as_str(), "")
    }
}

fn construct_xml_attributes_str_vec(xml_attributes_info: &XmlAttributeArrayinfo) -> Vec<String> {
    xml_attributes_info
        .attributes
        .iter()
        .map(|attr_vec| {
            attr_vec
                .iter()
                .map(|attr| {
                    if attr.xml_atrribute_key.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "{}=\"{}\"",
                            attr.xml_atrribute_key.clone(),
                            attr.xml_attribute_value.clone()
                        )
                    }
                })
                .join(" ")
        })
        .collect::<Vec<String>>()
}

fn construct_xml_attributes_str(xml_attributes_info: &XmlAttributeObjectInfo) -> String {
    xml_attributes_info
        .attributes
        .iter()
        .map(|attr| {
            if attr.xml_atrribute_key.is_empty() {
                String::new()
            } else {
                format!(
                    "{}=\"{}\"",
                    attr.xml_atrribute_key.clone(),
                    attr.xml_attribute_value.clone()
                )
            }
        })
        .join(" ")
}

pub fn remove_str_chunk_by_key(str_val: &mut String, str_key: &String) {
    let mut found_indices = str_val.match_indices(str_key);
    let mut found_indices_ints = found_indices
        .into_iter()
        .map(|(indx, _)| indx)
        .collect::<Vec<usize>>();

    while found_indices_ints.len() > 1 {
        *str_val = str_val
            .chars()
            .take(found_indices_ints[0])
            .chain(str_val.chars().skip(found_indices_ints[1] + str_key.len()))
            .collect();
        found_indices = str_val.match_indices(str_key);
        found_indices_ints = found_indices
            .into_iter()
            .map(|(indx, _)| indx)
            .collect::<Vec<usize>>();
    }
}
