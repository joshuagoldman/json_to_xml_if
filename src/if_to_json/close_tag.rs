use super::{
    json_build::json_construct, unexpected_character_error, ClosedTagStage, InitEndKeys, Node,
    NodeStage, OpenTagStage, State,
};

fn check_if_node_val_empty(state: &mut State) -> bool {
    let len = state.nodes.len() - 1;

    match state.nodes[len].node_result.clone() {
        super::ChildNodesOrKeyVal::KeyValue(node_res) => node_res.str_value == "null",
        super::ChildNodesOrKeyVal::ChildNodes(_) => false,
    }
}

pub fn closed_tag_value_stage_forward_slash(char_val: &char, state: &mut State) {
    match char_val {
        '>' => {
            if check_if_node_val_empty(state) {
                state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::AngelBracket));
            } else {
                unexpected_character_error(char_val, state)
            }
        }
        _ => {
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
    }
}

pub fn closed_tag_key_stage(
    char_val: &char,
    state: &mut State,
    open_tag_key: &String,
    close_tag_key: &String,
) {
    match char_val {
        '>' => {
            if open_tag_key == close_tag_key {
                state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::AngelBracket));
            }
        }
        _ => {
            let new_key = format!("{}{}", close_tag_key, char_val);
            state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::Key(InitEndKeys {
                open_tag_key: open_tag_key.to_owned(),
                closed_tag_key: new_key,
            })))
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
            let last_node = state.nodes[state.nodes.len() - 1].clone();
            match last_node.node_key {
                Some(some_open_tag_key) => {
                    state.update_node_stage(NodeStage::ClosedTag(ClosedTagStage::Key(
                        InitEndKeys {
                            open_tag_key: some_open_tag_key,
                            closed_tag_key: String::new(),
                        },
                    )));
                }
                None => panic!("No open tag was found!"),
            }
        }
        _ => {
            json_construct(state);
            state.nodes.pop();
            let mut node = Node::new();
            node.node_key = Some(char_val.to_string());
            state.nodes.push(node);
            state.update_node_stage(NodeStage::OpenTag(OpenTagStage::Key));
        }
    }
}
