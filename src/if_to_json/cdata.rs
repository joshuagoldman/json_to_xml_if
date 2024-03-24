use regex::Regex;

use super::{
    unexpected_character_error, ClosedTagStage, KeyStage, Node, NodeStage, NodeType, OpenTagStage,
    State, ValueStage, XmlAttributeStage,
};

pub fn open_tag_stage_cdata_open(char_val: &char, state: &mut State, curr_val: &String) {
    let mut new_string_val = format!("{}{}", curr_val, char_val);
    match char_val {
        '<' => {
            if new_string_val.ends_with("]]>") {
                state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::ForwardSlash));
                state.update_is_nested(false);
            } else {
                state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValueCData(
                    new_string_val,
                )))
            }
        }
        _ => state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValueCData(
            new_string_val,
        ))),
    }
}

pub fn open_tag_stage_cdata_closed(char_val: &char, state: &mut State) {
    match char_val {
        '/' => {
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::Key));
        }
        _ => unexpected_character_error(char_val, state),
    }
}
