use regex::Regex;

use super::{
    open_tag::key_update, unexpected_character_error, ClosedTagStage, Node, NodeStage,
    OpenTagStage, State, XmlAttribute,
};

pub fn closed_tag_value_stage_forward_slash(char_val: &char, state: &mut State, node_val: &String) {
    match char_val {
        '/' => {
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::Key));
        }
        _ => {
            state.nodes.push(Node::new());

            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key));
            key_update(state, char_val);
        }
    }
}

pub fn closed_tag_key_stage(char_val: &char, state: &mut State) {
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    match char_val {
        '>' => {
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::AngelBracket));
        }
        _ => {
            let new_key = key_update(state, char_val);
            if let None = regex.captures(new_key.as_str()) {
                panic!("unexpected tag key name at row {}", state.curr_row_num)
            }
        }
    }
}

pub fn closed_tag_angle_bracket(char_val: &char, state: &mut State) {
    match char_val {
        '<' => {
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::SibingOrClosing));
        }
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn closed_tag_sibling_or_closing(char_val: &char, state: &mut State) {
    match char_val {
        '/' => {
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::SibingOrClosing));
        }
        _ => unexpected_character_error(char_val, state),
    }
}

fn get_xml_attributes(
    key: &String,
    attributes: &Vec<XmlAttribute>,
    state: &State,
    attr_str: &mut String,
) -> bool {
    let indent_str = state.get_indentation_str();
    if attributes.len() > 0 {
        let attribute_chunks = attributes
            .iter()
            .map(|attr| {
                format!(
                    "{}{{\n{}\"key\": \"{}\",\n{}\"value\": \"{}\"\n{}}}",
                    indent_str,
                    indent_str,
                    attr.attribute_key.clone(),
                    indent_str,
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

fn build_array_json(key: &String, array_vals: &Vec<String>, state: &State) -> String {
    let indent_str = state.get_indentation_str();

    let array_components = array_vals.join(",\n");
    let mut attr_str = String::new();
    let xml_attributes = state.nodes[state.nodes.len() - 1].xml_attributes;

    if get_xml_attributes(key, &xml_attributes, state, &mut attr_str) {
        format!(
            "{}\"{}\": [\n{}\n{}],\n{}",
            indent_str, key, array_components, indent_str, attr_str
        )
    } else {
        format!(
            "{}\"{}\": [\n{}\n{}]",
            indent_str, key, array_components, indent_str
        )
    }
}

fn build_object_json(key: &String, object_vals: &String, state: &State) -> String {
    let indent_str = state.get_indentation_str();

    let mut attr_str = String::new();
    let xml_attributes = state.nodes[state.nodes.len() - 1].xml_attributes;

    if get_xml_attributes(key, &xml_attributes, state, &mut attr_str) {
        format!(
            "{}\"{}\": {{\n{},\n{}\n {}}}",
            indent_str, key, object_vals, attr_str, indent_str
        )
    } else {
        format!(
            "{}\"{}\": {{\n{}\n{}}}",
            indent_str, key, object_vals, indent_str
        )
    }
}

fn build_string_json(key: &String, str_val: &String, state: &State) -> String {
    let indent_str = state.get_indentation_str();

    let mut attr_str = String::new();
    let xml_attributes = state.nodes[state.nodes.len() - 1].xml_attributes;

    if get_xml_attributes(key, &xml_attributes, state, &mut attr_str) {
        format!("{}\"{}\": \"{}\",\n{}", indent_str, key, str_val, attr_str)
    } else {
        format!("{}\"{}\": \"{}\"", indent_str, key, str_val)
    }
}

fn build_num_json(key: &String, num_val: &i32, state: &State) -> String {
    let indent_str = state.get_indentation_str();

    let mut attr_str = String::new();
    let xml_attributes = state.nodes[state.nodes.len() - 1].xml_attributes;

    if get_xml_attributes(key, &xml_attributes, state, &mut attr_str) {
        format!("{}\"{}\": {},\n{}", indent_str, key, num_val, attr_str)
    } else {
        format!("{}\"{}\": {}", indent_str, key, num_val)
    }
}

pub fn sibling_close_decision(char_val: &char, state: &mut State) {
    let len = state.nodes.len() - 1;
    let key = match state.nodes[len].node_key {
        Some(some_key) => some_key,
        None => "parameters".to_string(),
    };
    let node_as_json_str = match state.nodes[len.clone()].node_type {
        super::NodeType::Array(array_vals) => build_array_json(&key, &array_vals, &state),
        super::NodeType::Object(object_vals) => build_object_json(&key, &object_vals, state),
        super::NodeType::Number(num_val) => build_num_json(&key, &num_val, state),
        super::NodeType::String(str_val) => build_string_json(&key, &str_val, state),
    };

    match state.nodes[len - 2].node_type {
        super::NodeType::Array(_) => todo!(),
        super::NodeType::Object(_) => todo!(),
        super::NodeType::Number(_) => todo!(),
        super::NodeType::String(_) => todo!(),
    }
}
