use regex::Regex;

use super::{
    json_build::add_key_val_node_result, unexpected_character_error, ClosedTagStage, InitEndKeys,
    Node, NodeStage, OpenTagStage, State, ValueStage, XmlAttributeStage,
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

pub fn open_tag_key_stage_open(char_val: &char, state: &mut State, is_white_space: bool) {
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    if is_white_space {
        let last_node = state.nodes[state.nodes.len() - 1].clone();
        if last_node.node_key.is_some() {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Attributes(
                XmlAttributeStage::AttributeKey(ValueStage::Open(String::new())),
            )));
        }
    } else {
        match char_val {
            '>' => {
                let last_node = state.nodes[state.nodes.len() - 1].clone();
                if last_node.node_key.is_none() {
                    unexpected_character_error(char_val, state)
                }
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
            _ => {
                let new_key = key_update(state, char_val);
                if let None = regex.captures(new_key.as_str()) {
                    panic!(
                        "unexpected tag key name {} at row {}",
                        state.curr_row_num, new_key
                    )
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
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(
                char_val.to_string(),
            )));
        }
    }
}

pub fn closed_key_is_empty_value(char_val: &char, state: &mut State) {
    match char_val {
        '/' => {
            add_key_val_node_result(state, &String::new());
            let last_node = state.nodes[state.nodes.len() - 1].clone();
            match last_node.node_key {
                Some(some_open_tag_key) => state.update_node_stage(NodeStage::ClosedTag(
                    ClosedTagStage::Key(InitEndKeys {
                        open_tag_key: some_open_tag_key,
                        closed_tag_key: String::new(),
                    }),
                )),
                None => panic!("No open tag was found!"),
            }
        }
        _ => {
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(format!(
                "<{}",
                char_val
            ))));
        }
    }
}

pub fn open_tag_value_stage(char_val: &char, state: &mut State, node_val: &String) {
    let regex = Regex::new(r"^<[aA-zZ]").unwrap();
    let new_string_val = format!("{}{}", node_val, char_val);
    let is_match = regex.is_match(new_string_val.as_str());

    if new_string_val.starts_with("<!") && new_string_val.len() == 3 {
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValueCData(
            new_string_val,
        )));
    } else if is_match {
        state.nodes.push(Node::new());
        state.curr_indent += 1;
        let len = state.nodes.len() - 1;
        let mut new_string_split_off = new_string_val.clone();
        new_string_split_off = new_string_split_off.split_off(1);
        state.nodes[len.clone()].node_key = Some(new_string_split_off);
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key));
    } else if char_val == &'<' {
        add_key_val_node_result(state, &node_val);
        state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::ClosedTagOpening));
    } else {
        state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(new_string_val)));
    }
}
