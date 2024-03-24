use regex::Regex;

use super::{
    ClosedTagStage, KeyStage, Node, NodeStage, NodeType, OpenTagStage, State, ValueStage,
    XmlAttributeStage,
};

pub fn key_update(state: &mut State, char_val: &char) -> String {
    let len = state.nodes.len() - 1;
    match state.nodes[len.clone()].node_key.clone() {
        Some(some_key) => {
            let new_key = format!("{}{}", some_key, char_val);
            state.nodes[len].node_key = Some(new_key.clone());
            new_key
        }
        None => {
            state.nodes[len].node_key = Some(char_val.to_string());
            char_val.to_string()
        }
    }
}

pub fn open_tag_init(char_val: &char, state: &mut State) {
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    if let None = regex.captures(char_val.to_string().as_str()) {
        panic!("unexpected tag key name at row {}", state.curr_row_num)
    }
    let _ = key_update(state, char_val);
    state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key));
}

pub fn open_tag_key_stage_open(char_val: &char, state: &mut State, is_white_space: bool) {
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    if is_white_space {
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
            XmlAttributeStage::AttributeKey(ValueStage::Open(String::new())),
        )));
    } else {
        match char_val {
            '>' => {
                state.update_node_stage(NodeStage::OpenTag(OpenTagStage::AngelBracket));
            }
            _ => {
                let new_key = key_update(state, char_val);
                if let None = regex.captures(new_key.as_str()) {
                    panic!("unexpected tag key name at row {}", state.curr_row_num)
                }
            }
        }
    }
}

pub fn closed_key_is_angle_bracket(char_val: &char, state: &mut State) {
    match char_val {
        '<' => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::IsEmptyValue));
        }
        _ => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(String::new())));
        }
    }
}

pub fn closed_key_is_empty_value(char_val: &char, state: &mut State) {
    match char_val {
        '/' => {
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::Key));
        }
        _ => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(String::new())));
        }
    }
}

pub fn open_tag_value_stage(char_val: &char, state: &mut State, node_val: &String) {
    let new_string_val = format!("{}{}", node_val, char_val);
    if new_string_val.starts_with("<!") {
        if new_string_val == "![CDATA[" {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValueCData(
                String::new(),
            )));
        } else {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(new_string_val)));
        }
    } else if new_string_val.starts_with("<") {
        state.nodes.push(Node::new());
        state.update_is_nested(true);
        state.curr_indent += 1;
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key));
    } else if new_string_val.len() > 1 && char_val == &'<' {
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(new_string_val)));
        state.update_is_nested(false);
    } else {
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(new_string_val)));
    }
}
