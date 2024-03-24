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
            "{}\"{}\": {{\n{}\n{},\n{}",
            indent_str, node.key, node.str_value, indent_str, xml_attributes_str_format
        )
    } else {
        format!(
            "{}\"{}\": [\n{}\n{}]",
            indent_str, node.key, node.str_value, indent_str
        )
    }
}

fn build_string_json(node: &NodeStrResult, state: &mut State) -> String {
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
            "{}\"{}\": \"{}\",\n{}{}",
            indent_str, node.key, node.str_value, indent_str, xml_attributes_str_format
        )
    } else {
        format!("{}\"{}\": \"{}\"", indent_str, node.key, node.str_value)
    }
}

fn build_num_json(node: &NodeStrResult, num: &i32, state: &mut State) -> String {
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
            "{}\"{}\": {},\n{}{}",
            indent_str, node.key, num, indent_str, xml_attributes_str_format
        )
    } else {
        format!("{}\"{}\": {}", indent_str, num, node.str_value)
    }
}

fn node_to_json(nodes: &Vec<NodeStrResult>, state: &mut State) -> String {
    const RADIX: u32 = 10;
    match nodes.len() > 1 {
        true => build_array_json(nodes, state),
        false => {
            let mut node = nodes[0].clone();
            if node.str_value.starts_with("<![CDATA[") && node.str_value.ends_with("]]>") {
                let _ = node.str_value.split_off(node.str_value.len() - 3);
                node.str_value = node.str_value.split_off(9);
            }
            match (node.is_nested, node.str_value.parse::<i32>()) {
                (true, _) => build_object_json(&node, state),
                (false, Ok(num)) => build_num_json(&node, &num, state),
                (false, Err(_)) => build_string_json(&node, state),
            }
        }
    }
}

pub fn sibling_close_decision(state: &mut State) {
    let len = state.nodes.len() - 1;

    let child_nodes_as_str = state.nodes[len]
        .clone()
        .child_nodes
        .iter()
        .group_by(|x| x.key.clone())
        .into_iter()
        .map(|(_, group)| {
            let group_as_vec = group.map(|x| x.to_owned()).collect::<Vec<NodeStrResult>>();
            node_to_json(&group_as_vec, state)
        })
        .collect::<Vec<String>>()
        .join(",\n");

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
        state.update_child_nodes(NodeStrResult {
            str_value: child_nodes_as_str,
            xml_attributes_str,
            key,
            is_nested: last_node.is_nested,
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
