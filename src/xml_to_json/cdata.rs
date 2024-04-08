use super::{json_build::add_key_val_node_result, ClosedTagStage, NodeStage, OpenTagStage, State};

pub fn open_tag_stage_cdata_open(char_val: &char, state: &mut State, curr_val: &String) {
    let new_string_val = format!("{}{}", curr_val, char_val);
    match char_val {
        '<' => {
            if curr_val.trim_end().ends_with("]]>") && curr_val.starts_with("<![CDATA[") {
                state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::ForwardSlash));
                add_key_val_node_result(state, &curr_val.trim().to_string());
            } else {
                state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValueCData(
                    new_string_val,
                )))
            }
        }
        _ => {
            if "<![CDATA[".contains(new_string_val.as_str())
                | new_string_val.starts_with("<![CDATA[")
            {
                state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValueCData(
                    new_string_val,
                )))
            } else {
                state.update_node_stage(NodeStage::OpenTag(OpenTagStage::TagValue(new_string_val)))
            }
        }
    }
}
