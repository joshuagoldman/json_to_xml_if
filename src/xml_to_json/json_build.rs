use convert_case::Casing;
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

fn increase_curly_braces_indent_for_array(node: &NodeStrResult) -> String {
    let mut str_val = node.str_value.clone();
    str_val = str_val.replacen("{", "  {", 1);
    str_val = str_val.replace("\n", "\n ");
    str_val
}

pub fn build_array_json(nodes: &Vec<NodeStrResult>, state: &mut State) -> String {
    let indent_str = state.get_indentation_str();
    let indent_str_inner = format!("{} ", indent_str);
    let key = nodes[0].key.clone();
    let key_formated = if state.to_camel_case {
        key.to_case(convert_case::Case::Camel)
    } else {
        key.clone()
    };

    let array_components = nodes
        .iter()
        .map(|x| match x.is_object {
            true => increase_curly_braces_indent_for_array(x),
            false => x.str_value.clone(),
        })
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

    let all_attr_null = xml_attributes.iter().all(|x| x.trim() == "null");
    if !all_attr_null {
        let xml_attributes_str_format = format!(
            "{}\"{}_attributes\": [\n{}\n{}]",
            indent_str_inner, key_formated, xml_attributes_str, indent_str_inner
        );
        format!(
            "{}\"{}\": [\n{}\n{}],\n{}",
            indent_str_inner,
            key_formated,
            array_components,
            indent_str_inner,
            xml_attributes_str_format
        )
    } else {
        format!(
            "{}\"{}\": [\n{}\n{}]",
            indent_str_inner, key_formated, array_components, indent_str_inner
        )
    }
}

pub fn build_object_json(node: &NodeStrResult, state: &mut State) -> String {
    let indent_str = format!("{} ", state.get_indentation_str());

    let mut xml_attributes_str = node.xml_attributes_str.clone();

    if xml_attributes_str.is_empty() {
        xml_attributes_str = "null".to_string();
    }

    let mut xml_attributes_str_new = xml_attributes_str.trim_start().trim_end().to_string();
    xml_attributes_str_new.remove(xml_attributes_str_new.len() - 3);

    let str_value_new = node.str_value.trim_start().to_string();

    let key_formated = if state.to_camel_case {
        node.key.to_case(convert_case::Case::Camel)
    } else {
        node.key.clone()
    };

    if xml_attributes_str != "null" {
        let xml_attributes_str_format = format!(
            "{}\"{}_attributes\": {}",
            indent_str, key_formated, xml_attributes_str_new
        );
        format!(
            "{}\"{}\": {},\n{}",
            indent_str, key_formated, str_value_new, xml_attributes_str_format
        )
    } else {
        format!("{}\"{}\": {}", indent_str, key_formated, str_value_new)
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
    } else if let Ok(_) = str_val.parse::<i32>() {
        format!("{}", str_val.clone())
    } else {
        format!("\"{}\"", str_val.clone())
    };

    let new_str_indented = format!("{}  {}", state.get_indentation_str(), new_str);
    let node_res = NodeStrResult {
        str_value: new_str_indented,
        xml_attributes_str,
        key,
        is_object: false,
    };

    state.update_node_key_val(node_res);
}

fn child_nodes_or_key_val_handling(state: &mut State) -> NodeStrResult {
    let len = state.nodes.len() - 1;
    let last_node = state.nodes[len].clone();
    let key = match last_node.node_key {
        Some(some_key) => some_key,
        None => "unknown".to_string(),
    };
    let mut xml_attributes_str = String::new();
    get_xml_attributes(&last_node.xml_attributes, state, &mut xml_attributes_str);
    match state.nodes[len].clone().child_node_result {
        super::ChildNodesOrKeyVal::KeyValue(node_res) => node_res,
        super::ChildNodesOrKeyVal::ChildNodes(child_nodes) => {
            let chilren_as_str = child_nodes
                .iter()
                .group_by(|x| x.key.clone())
                .into_iter()
                .map(|(_, group)| {
                    let group_as_vec = group.map(|x| x.to_owned()).collect::<Vec<NodeStrResult>>();
                    node_to_json(&group_as_vec, state)
                })
                .collect::<Vec<String>>()
                .join(",\n");

            let indent_str = format!("{}", state.get_indentation_str());
            let final_node_str = format!("{{\n{}\n{}}}", chilren_as_str, indent_str);
            NodeStrResult {
                str_value: final_node_str.clone(),
                xml_attributes_str,
                key,
                is_object: true,
            }
        }
    }
}

pub fn json_construct(state: &mut State) {
    let node = child_nodes_or_key_val_handling(state);

    state.update_node_result_parent(node.clone());
    state.nodes.pop();
}

pub fn construct_last(state: &mut State) {
    let node = child_nodes_or_key_val_handling(state);
    state.str_json = Some(build_object_json(&node, state).trim_start().to_string());
}
