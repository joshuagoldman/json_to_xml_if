use regex::Regex;

use super::{
    unexpected_character_error, KeyStage, Node, NodeStage, OpenTagStage, State, ValueStage,
    XmlAttributeStage,
};

pub fn xml_attribute_key_open(char_val: &char, state: &mut State, curr_xml_attr_key: &String) {
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    let new_key = format!("{}{}", curr_xml_attr_key, char_val);
    match char_val {
        '=' => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeKey(ValueStage::Closed),
            )));
            state.update_attribute_key(&curr_xml_attr_key);
        }
        '>' => {
            if !curr_xml_attr_key.is_empty() {
                unexpected_character_error(char_val, state)
            }
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key))
        }
        _ => {
            if let None = regex.captures(new_key.as_str()) {
                panic!("unexpected attribute key at row {}", state.curr_row_num)
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
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    let new_key = format!("{}{}", curr_xml_attr_val, char_val);
    match char_val {
        '"' => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeValue(ValueStage::Closed),
            )));
            state.update_attribute_val(curr_xml_attr_val);
        }
        _ => {
            if let None = regex.captures(new_key.as_str()) {
                panic!(
                    "unexpected attribute key value at row {}",
                    state.curr_row_num
                )
            }

            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeKey(ValueStage::Open(new_key)),
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
                state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key));
            }
            _ => unexpected_character_error(char_val, state),
        }
    }
}
