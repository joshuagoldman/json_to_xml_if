use crate::hebrew_handler::hebrew_str_to_non_hebrew;

#[derive(Clone, Debug)]
pub struct StoredProcedureInfo {
    pub procedureName: String,
}

#[derive(Clone, Debug)]
pub enum ParameterDirection {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub enum OracleDbType {
    Varchar2,
    RefCursor,
}

#[derive(Clone, Debug)]
pub struct StoredProcedureParameter {
    pub param_name: String,
    pub param_value: String,
    pub param_direction: ParameterDirection,
    pub param_type: OracleDbType,
    pub position: int,
}

#[derive(Clone, Debug)]
pub struct StoredProcedure {
    pub info: StoredProcedureInfo,
    pub params: Vec<StoredProcedureParameter>,
}

#[derive(Debug, Clone)]
pub enum ProcVariableStages {
    VariableName(String),
    VariableDirection(String),
    VariableType(String),
}

#[derive(Debug, Clone)]
pub enum ProcDecalarationStage {
    ProcedureKeyWord,
    ProcedureName(String),
    OpenBracket,
    Variable(ProcVariableStages),
    VariableSeparator,
    CloseBracket,
}

#[derive(Debug)]
pub struct State {
    pub meta_data_json_arr: String,
    pub request_json_arr: String,
    pub storedProcedures: Vec<StoredProcedure>,
}

impl State {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            curr_row_num: 1,
            curr_indent: 0,
            str_json: None,
            to_camel_case: true,
        }
    }
}

fn unexpected_character_error(char_val: &char, state: &State) {
    print!("{:#?}", state);
    panic!(
        "Unexpected character '{}' at row {}",
        char_val, state.curr_row_num
    )
}

fn is_white_space(char_val: &char) -> bool {
    vec![' ', '\n', '\t', '\r'].iter().any(|x| x == char_val)
}

fn should_not_ignore_white_space(char_val: &char, state: &mut State) -> bool {
    if !is_white_space(char_val) {
        return false;
    }
    if let None = state.nodes[state.nodes.len() - 1].node_key {
        return true;
    }

    let last_node = state.nodes[state.nodes.len() - 1].stage.clone();
    match last_node {
        NodeStage::OpenTag(OpenTagStage::Key) => {
            open_tag_key_stage_open(char_val, state, true);
        }
        NodeStage::OpenTag(OpenTagStage::Attributes(XmlAttributeStage::AttributeValue(
            ValueStage::Closed,
        ))) => {
            xml_attribute_value_closed(char_val, state, true);
        }
        NodeStage::OpenTag(OpenTagStage::TagValue(_))
        | NodeStage::OpenTag(OpenTagStage::TagValueCData(_))
        | NodeStage::OpenTag(OpenTagStage::Attributes(XmlAttributeStage::AttributeValue(
            ValueStage::Open(_),
        ))) => {
            return false;
        }
        _ => (),
    }

    true
}

fn to_json(char_val: &char, state: &mut State) {
    if vec!['\n'].iter().any(|x| x == char_val) {
        state.curr_row_num += 1;
    }

    if state.nodes.len() == 0 {
        if is_white_space(char_val) {
            return;
        }

        if char_val != &'<' {
            unexpected_character_error(char_val, state)
        }
        state.nodes.push(Node::new());
        return;
    }

    if should_not_ignore_white_space(char_val, state) {
        return;
    }

    let node_stage = state.nodes[state.nodes.len() - 1].clone().stage.clone();
    match node_stage.clone() {
        NodeStage::OpenTag(open_tag_options) => match open_tag_options {
            OpenTagStage::Key => open_tag_key_stage_open(char_val, state, false),
            OpenTagStage::Attributes(open_tag_stage_attributes) => {
                open_tag_stage_attributes_decision(char_val, state, open_tag_stage_attributes)
            }
            OpenTagStage::TagValueCData(curr_val) => {
                open_tag_stage_cdata_open(char_val, state, &curr_val)
            }
            OpenTagStage::TagValue(node_val) => open_tag_value_stage(char_val, state, &node_val),
            OpenTagStage::AngelBracket => closed_key_is_angle_bracket(char_val, state),
            OpenTagStage::IsEmptyValue => closed_key_is_empty_value(char_val, state),
        },
        NodeStage::ClosedTag(closed_tag_options) => match closed_tag_options {
            ClosedTagStage::ClosedTagOpening => closed_tag_opening(char_val, state),
            ClosedTagStage::ForwardSlash => closed_tag_value_stage_forward_slash(char_val, state),
            ClosedTagStage::Key(init_end_keys) => closed_tag_key_stage(
                char_val,
                state,
                &init_end_keys.open_tag_key,
                &init_end_keys.closed_tag_key,
            ),
            ClosedTagStage::AngelBracket => closed_tag_angle_bracket(char_val, state),
            ClosedTagStage::SibingOrClosing => closed_tag_sibling_or_closing(char_val, state),
        },
    }
}

pub fn pl_sql_to_json(pl_sql_cntnt: &String) -> Result<String, String> {
    let mut state = State::new();
    state.to_camel_case = to_camel_case;
    let mut xml_str_no_hebrew = xml_str.clone();
    hebrew_str_to_non_hebrew(&mut xml_str_no_hebrew, true);
    for (_, char_val) in xml_str_no_hebrew.chars().enumerate() {
        to_if_req_single(&char_val, &mut state);
    }

    if let Some(res_json) = state.str_json {
        let mut ok_res = res_json
            .chars()
            .skip_while(|x| x != &':')
            .skip(1)
            .collect::<String>()
            .trim()
            .to_string();

        hebrew_str_to_non_hebrew(&mut ok_res, false);
        Result::Ok(ok_res)
    } else {
        Result::Err("Not enough data to construct a json".to_string())
    }
}
