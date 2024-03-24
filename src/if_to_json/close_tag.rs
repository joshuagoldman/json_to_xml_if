use iter_tools::Itertools;
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
            state.nodes.pop();
            state.curr_indent -= 1;
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::Key));
        }
        _ => {
            state.nodes.pop();
            state.nodes.push(Node::new());
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key));
        }
    }
}
