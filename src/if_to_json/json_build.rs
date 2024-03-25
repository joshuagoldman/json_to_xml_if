use iter_tools::Itertools;

use super::{NodeStrResult, State, XmlAttribute};

fn get_xml_attributes(
    key: &String,
    attributes: &Vec<XmlAttribute>,
    state: &mut State,
    attr_str: &mut String,
) -> bool {
    let indent_str = state.get_indentation_str();
    if attributes.len() > 0 {
        let attribute_chunks = attributes
            .iter()
            .map(|attr| {
                format!(
                    "{}{{\n{}\"{}\": \"{}\",\n{}}}",
                    indent_str,
                    indent_str,
                    attr.attribute_key.clone(),
                    attr.attribute_val.clone(),
                    indent_str
                )
            })
            .collect::<Vec<String>>()
            .join(",\n");

        *attr_str = format!(
            "{}\"{}\": [\n{}{}\n{}]",
            indent_str, key, indent_str, attribute_chunks, indent_str
        );

        true
    } else {
        false
    }
}

fn build_array_json(nodes: &Vec<NodeStrResult>, state: &mut State) -> String {
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

    let xml_attributes_str = xml_attributes
        .iter()
        .map(|x| format!("{}{{\n{}{}\n{}}}", indent_str, indent_str, x, indent_str))
        .collect::<Vec<String>>()
        .join(",\n");

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

fn build_object_json(node: &NodeStrResult, state: &mut State) -> String {
    let indent_str = state.get_indentation_str();

    let mut xml_attributes_str = node.xml_attributes_str.clone();

    if xml_attributes_str.is_empty() {
        xml_attributes_str = "null".to_string();
    }

    if xml_attributes_str != "null" {
        let xml_attributes_str_format = format!(
            "{}\"{}_attributes\": {{\n{}\n{}}}",
            indent_str, node.key, xml_attributes_str, indent_str
        );
        format!(
            "{}\"{}\": \n{}\n{},\n{}",
            indent_str, node.key, indent_str, node.str_value, xml_attributes_str_format
        )
    } else {
        format!(
            "{}\"{}\": [\n{}\n{}]",
            indent_str, node.key, node.str_value, indent_str
        )
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
            "{}\"{}_attributes\": {{\n{}\n{}}}",
            indent_str, node.key, xml_attributes_str, indent_str
        );
        format!(
            "{}\"{}\": \"{}\",\n{}{}",
            indent_str, node.key, json_val, indent_str, xml_attributes_str_format
        )
    } else {
        format!("{}\"{}\": \"{}\"", indent_str, node.key, node.str_value)
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

    let _ = get_xml_attributes(
        &key,
        &last_node.xml_attributes,
        state,
        &mut xml_attributes_str,
    );

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

    let child_nodes_as_str = child_nodes_or_key_val_handling(state);

    let last_node = state.nodes[len].clone();
    let key = match last_node.node_key {
        Some(some_key) => some_key,
        None => "unknown".to_string(),
    };

    if state.nodes.len() > 1 {
        let mut xml_attributes_str = String::new();
        get_xml_attributes(
            &key,
            &last_node.xml_attributes,
            state,
            &mut xml_attributes_str,
        );
        state.update_node_result_parent(NodeStrResult {
            str_value: child_nodes_as_str,
            xml_attributes_str,
            key,
        });
    } else {
        match state.curr_json.is_empty() {
            true => state.curr_json = format!("{{\n{}\n}}", child_nodes_as_str),
            false => {
                state.curr_json = format!("{},\n{{\n{}\n}}", state.curr_json, child_nodes_as_str)
            }
        }
    }
}
