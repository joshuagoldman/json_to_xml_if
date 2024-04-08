use regex::Regex;

use super::{
    json_build::add_key_val_node_result, unexpected_character_error, ClosedTagStage, NodeStage,
    OpenTagStage, State, ValueStage, XmlAttributeStage,
};

pub fn xml_attribute_key_open(char_val: &char, state: &mut State, curr_xml_attr_key: &String) {
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    let new_key = format!("{}{}", curr_xml_attr_key, char_val);
    match char_val {
        '=' => {
            if curr_xml_attr_key.is_empty() {
                unexpected_character_error(char_val, state)
            }
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeKey(ValueStage::Closed),
            )));
            state.update_attribute_key(&curr_xml_attr_key);
        }
        '>' => {
            if !curr_xml_attr_key.is_empty() {
                unexpected_character_error(char_val, state)
            }

            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::AngelBracket))
        }
        '/' => {
            if !curr_xml_attr_key.is_empty() {
                unexpected_character_error(char_val, state)
            }

            add_key_val_node_result(state, &String::new());
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::ForwardSlash));
        }
        _ => {
            if let None = regex.captures(new_key.as_str()) {
                panic!(
                    "unexpected tag key name {} at row {}",
                    new_key, state.curr_row_num,
                )
            }

            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeKey(ValueStage::Open(new_key)),
            )));
        }
    }
}

pub fn xml_attribute_key_closed(char_val: &char, state: &mut State) {
    match char_val {
        '"' => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeValue(ValueStage::Open(String::new())),
            )));
        }
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn xml_attribute_value_open(char_val: &char, state: &mut State, curr_xml_attr_val: &String) {
    let new_key = format!("{}{}", curr_xml_attr_val, char_val);
    match char_val {
        '"' => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeValue(ValueStage::Closed),
            )));
            state.update_attribute_val(curr_xml_attr_val);
        }
        _ => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeValue(ValueStage::Open(new_key)),
            )));
        }
    }
}

pub fn xml_attribute_value_closed(char_val: &char, state: &mut State, is_white_space: bool) {
    if is_white_space {
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
            XmlAttributeStage::AttributeKey(ValueStage::Open(String::new())),
        )));
    } else {
        match char_val {
            '>' => {
                state.update_node_stage(NodeStage::OpenTag(OpenTagStage::AngelBracket));
            }
            '/' => {
                let last_node = state.nodes[state.nodes.len() - 1].clone();
                if last_node.node_key.is_none() {
                    unexpected_character_error(char_val, state)
                }
                add_key_val_node_result(state, &String::new());
                state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::ForwardSlash));
            }
            _ => unexpected_character_error(char_val, state),
        }
    }
}
