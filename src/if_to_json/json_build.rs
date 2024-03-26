use std::fmt::format;

use iter_tools::Itertools;

use super::{NodeStrResult, State, XmlAttribute};

pub fn get_xml_attributes(
    attributes: &Vec<XmlAttribute>,
    state: &mut State,
    attr_str: &mut String,
) -> bool {
    let indent_str_inner = format!("{} ", state.get_indentation_str());
    let indent_str_inner_array = format!("{} ", indent_str_inner.clone());
    if attributes.len() > 0 {
        let attribute_chunks = attributes
            .iter()
            .map(|attr| {
                format!(
                    "{}\"{}\": \"{}\"",
                    indent_str_inner_array,
                    attr.attribute_key.clone(),
                    attr.attribute_val.clone(),
                )
            })
            .collect::<Vec<String>>()
            .join(",\n");

        *attr_str = format!(
            "{}{{\n{}\n{}}}",
            indent_str_inner, attribute_chunks, indent_str_inner
        );

        true
    } else {
        false
    }
}

pub fn build_array_json(nodes: &Vec<NodeStrResult>, state: &mut State) -> String {
    let indent_str = state.get_indentation_str();
    let key = nodes[0].key.clone();

    let array_components = nodes
        .iter()
        .map(|x| x.str_value.clone())
        .collect::<Vec<String>>()
        .join(",\n");
    let xml_attributes = nodes
        .iter()
        .map(|x| {
            if x.xml_attributes_str.is_empty() {
                "null".to_string()
            } else {
                x.xml_attributes_str.clone()
            }
        })
        .collect::<Vec<String>>();

    let xml_attributes_str = xml_attributes.join(",\n");

    let all_attr_null = xml_attributes.iter().all(|x| x == &"null");
    if !all_attr_null {
        let xml_attributes_str_format = format!(
            "{}\"{}_attributes\": [\n{}\n{}]",
            indent_str, key, xml_attributes_str, indent_str
        );
        format!(
            "{}\"{}\": [\n{}\n{}],\n{}",
            indent_str, key, array_components, indent_str, xml_attributes_str_format
        )
    } else {
        format!(
            "{}\"{}\": [\n{}\n{}]",
            indent_str, key, array_components, indent_str
        )
    }
}

pub fn build_object_json(node: &NodeStrResult, state: &mut State) -> String {
    let indent_str = state.get_indentation_str();

    let mut xml_attributes_str = node.xml_attributes_str.clone();

    if xml_attributes_str.is_empty() {
        xml_attributes_str = "null".to_string();
    }

    let mut new_node = node.str_value.trim_start().trim_end().to_string();
    new_node.remove(new_node.len() - 3);
    let mut xml_attributes_str_new = xml_attributes_str.trim_start().trim_end().to_string();
    xml_attributes_str_new.remove(xml_attributes_str_new.len() - 3);
    if xml_attributes_str != "null" {
        let xml_attributes_str_format = format!(
            "{}\"{}_attributes\": {}",
            indent_str, node.key, xml_attributes_str_new
        );
        format!(
            "{}\"{}\": {},\n{}",
            indent_str, node.key, new_node, xml_attributes_str_format
        )
    } else {
        format!("{}\"{}\": \n{}", indent_str, node.key, node.str_value)
    }
}

fn build_json_simple_val(node: &NodeStrResult, state: &mut State) -> String {
    let indent_str = state.get_indentation_str();

    let mut xml_attributes_str = node.xml_attributes_str.clone();

    if xml_attributes_str.is_empty() {
        xml_attributes_str = "null".to_string();
    }

    let json_val = match (node.str_value == "null", node.str_value.parse::<i32>()) {
        (false, Ok(_)) => node.str_value.clone(),
        (false, Err(_)) => {
            format!("\"{}\"", node.str_value)
        }
        (_, _) => node.str_value.clone(),
    };

    if xml_attributes_str != "null" {
        let xml_attributes_str_format = format!(
            "{}\"{}_attributes\": {}",
            indent_str, node.key, xml_attributes_str
        );
        format!(
            "{}\"{}\": {},\n{}{}",
            indent_str, node.key, json_val, indent_str, xml_attributes_str_format
        )
    } else {
        format!("{}\"{}\": {}", indent_str, node.key, node.str_value)
    }
}

fn node_to_json(nodes: &Vec<NodeStrResult>, state: &mut State) -> String {
    match nodes.len() > 1 {
        true => build_array_json(nodes, state),
        false => build_object_json(&nodes[0], state),
    }
}

pub fn add_key_val_node_result(state: &mut State, str_val: &String) {
    let len = state.nodes.len() - 1;
    let last_node = state.nodes[len].clone();
    let key = match last_node.node_key {
        Some(some_key) => some_key,
        None => "unknown".to_string(),
    };
    let mut xml_attributes_str = String::new();

    let _ = get_xml_attributes(&last_node.xml_attributes, state, &mut xml_attributes_str);

    let new_str = if str_val.is_empty() {
        "null".to_string()
    } else {
        str_val.clone()
    };

    let node_res = NodeStrResult {
        str_value: new_str,
        xml_attributes_str,
        key,
    };

    state.update_node_result(node_res);
}

fn child_nodes_or_key_val_handling(state: &mut State) -> String {
    let len = state.nodes.len() - 1;
    match state.nodes[len].clone().node_result {
        super::ChildNodesOrKeyVal::KeyValue(node_res) => build_json_simple_val(&node_res, state),
        super::ChildNodesOrKeyVal::ChildNodes(child_nodes) => child_nodes
            .iter()
            .group_by(|x| x.key.clone())
            .into_iter()
            .map(|(_, group)| {
                let group_as_vec = group.map(|x| x.to_owned()).collect::<Vec<NodeStrResult>>();
                node_to_json(&group_as_vec, state)
            })
            .collect::<Vec<String>>()
            .join(",\n"),
    }
}

pub fn json_construct(state: &mut State) {
    let len = state.nodes.len() - 1;

    let final_node_str = child_nodes_or_key_val_handling(state);

    let last_node = state.nodes[len].clone();
    let key = match last_node.node_key {
        Some(some_key) => some_key,
        None => "unknown".to_string(),
    };

    if state.nodes.len() > 1 {
        let mut xml_attributes_str = String::new();
        get_xml_attributes(&last_node.xml_attributes, state, &mut xml_attributes_str);
        state.update_node_result_parent(NodeStrResult {
            str_value: final_node_str,
            xml_attributes_str,
            key,
        });
    } else {
        match state.curr_json.is_empty() {
            true => state.curr_json = format!("{{\n{}\n}}", final_node_str),
            false => state.curr_json = format!("{},\n{{\n{}\n}}", state.curr_json, final_node_str),
        }
    }
}
