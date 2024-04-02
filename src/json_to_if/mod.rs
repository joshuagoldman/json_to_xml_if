use self::{
    array_val::{
        array_val_json_null_case_closed, array_val_json_null_case_open,
        array_val_json_number_open_case, array_val_json_str_close_case,
        array_val_json_str_open_case,
    },
    json_array::{json_array_closed_case, json_array_item_separator_case, json_array_open_case},
    json_object::{
        json_object_closed_case, json_object_item_separator_case, json_object_open_case,
    },
    key_val::{
        key_closed_cased, key_open_case, key_val_json_null_case_closed,
        key_val_json_null_case_open, key_val_json_number_open_case, key_val_json_str_close_case,
        key_val_json_str_open_case, key_val_separator_case,
    },
    models::{
        ArrayValType, Field, JsonNull, JsonStr, KeyValState, KeyValType, NestingState, TokenStage,
        TokenStageKey, TokenType,
    },
    state::State,
    xml_attributes::{xml_attributes_check_state, xml_attributes_marking::get_attributes_mark},
};

pub mod array_val;
mod json_array;
mod json_object;
mod key_val;
pub mod models;
pub mod state;
pub mod xml_attributes;

fn add_open_tag(state: &mut State, indent: bool) {
    state.curr_indent += 1;
    let key = if state.fields.len() == 1 {
        "parameters".to_string()
    } else {
        state.fields[state.fields.len() - 1]
            .key
            .clone()
            .unwrap_or("unknown".to_string())
    };
    let indentation_str = if indent {
        state.get_indentation_str()
    } else {
        "".to_string()
    };

    let parent_index = state.fields.len() - 2;
    if state.fields.len() > 1 {
        if let NestingState::JsonArrayNestingState = state.fields[parent_index].nesting_state {
            state.curr_xml = format!("{}{}<{}>", state.curr_xml, indentation_str, key);
        }
    } else if let Some(ids_info) = state.check_init_xml_attributes() {
        state.curr_xml = format!(
            "{}{}{}<{} {}>",
            state.curr_xml, indentation_str, ids_info.attr_object_id, key, ids_info.attr_id
        );
    } else if let Some(attr_id) = get_attributes_mark(state, &key) {
        state.curr_xml = format!("{}{}<{} {}>", state.curr_xml, indentation_str, key, attr_id);
    } else {
        state.curr_xml = format!("{}{}<{}>", state.curr_xml, indentation_str, key);
    }
}

fn add_close_tag(state: &mut State, indent: bool) {
    let key = if state.fields.len() == 1 {
        "parameters".to_string()
    } else {
        state.fields[state.fields.len() - 1]
            .key
            .clone()
            .unwrap_or("unknown".to_string())
    };
    let indentation_str = if indent {
        state.get_indentation_str()
    } else {
        "".to_string()
    };
    state.curr_xml = format!("{}{}</{}>", state.curr_xml, indentation_str, key);
    let parent_index = state.fields.len() - 2;
    if state.fields.len() > 1 {
        if let NestingState::JsonArrayNestingState = state.fields[parent_index].nesting_state {
            state.curr_xml = format!("{}{}</{}>", state.curr_xml, indentation_str, key);
        }
    } else if let Some(obj_id) = state.get_obj_id_for_closing_tag() {
        state.curr_xml = format!("{}{}</{}>{}", state.curr_xml, indentation_str, key, obj_id);
    } else {
        state.curr_xml = format!("{}{}</{}>", state.curr_xml, indentation_str, key);
    }
    state.curr_indent -= 1;
}

fn add_tag_val(state: &mut State, str_val: &String) {
    state.curr_xml = format!("{}{}", state.curr_xml, str_val);
}

fn unexpected_character_error(char_val: &char, state: &State) {
    print!("{:#?}", state);
    panic!(
        "Unexpected character '{}' at row {}",
        char_val, state.curr_row_num
    )
}

fn key_val_state_key_state_decision(
    key_val_state_key_state: TokenStageKey,
    char_val: &char,
    state: &mut State,
) {
    match key_val_state_key_state {
        TokenStageKey::Opening => key_open_case(char_val, state),
        TokenStageKey::KeyValSeparator => key_val_separator_case(char_val, state),
        TokenStageKey::Closing => key_closed_cased(char_val, state),
    }
}

fn key_val_state_val_decision(key_val_state_val: KeyValType, char_val: &char, state: &mut State) {
    match key_val_state_val {
        KeyValType::JsonStr(json_str_val) => match json_str_val {
            JsonStr::Open(json_str_val) => {
                key_val_json_str_open_case(char_val, state, &json_str_val)
            }
            JsonStr::Closing => key_val_json_str_close_case(char_val, state),
        },
        KeyValType::JsonNumber(json_number) => {
            key_val_json_number_open_case(char_val, state, &json_number)
        }
        KeyValType::Null(json_null) => match json_null {
            JsonNull::Open(null_str_val) => {
                key_val_json_null_case_open(char_val, state, &null_str_val)
            }
            JsonNull::Closing => key_val_json_null_case_closed(char_val, state),
        },
    }
}

fn token_stage_content_decision(
    token_stage_content: ArrayValType,
    char_val: &char,
    state: &mut State,
) {
    match token_stage_content {
        ArrayValType::JsonStr(json_str) => match json_str {
            JsonStr::Open(json_str_val) => {
                array_val_json_str_open_case(char_val, state, &json_str_val)
            }
            JsonStr::Closing => array_val_json_str_close_case(char_val, state),
        },
        ArrayValType::JsonNumber(json_num_str) => {
            array_val_json_number_open_case(char_val, state, &json_num_str)
        }
        ArrayValType::Null(null_state) => match null_state {
            JsonNull::Open(json_str_val) => {
                array_val_json_null_case_open(char_val, state, &json_str_val)
            }
            JsonNull::Closing => array_val_json_null_case_closed(char_val, state),
        },
    }
}

fn token_stage_content_key_val_decision(
    token_stage_content_key_val: KeyValState,
    char_val: &char,
    state: &mut State,
) {
    match token_stage_content_key_val {
        KeyValState::KeyState(key_val_state_key_state) => {
            key_val_state_key_state_decision(key_val_state_key_state, char_val, state)
        }
        KeyValState::ValState(key_val_state_val) => {
            key_val_state_val_decision(key_val_state_val, char_val, state)
        }
    }
}

fn token_type_json_object_decision(
    token_type_json_object: TokenStage<KeyValState>,
    char_val: &char,
    state: &mut State,
) {
    match token_type_json_object {
        TokenStage::Opening => json_object_open_case(char_val, state),
        TokenStage::Content(token_stage_content) => {
            token_stage_content_key_val_decision(token_stage_content, char_val, state)
        }
        TokenStage::ItemSeparator => json_object_item_separator_case(char_val, state),
        TokenStage::Closing => json_object_closed_case(char_val, state),
    }
}

fn token_type_json_array_decision(
    token_type_json_array: TokenStage<ArrayValType>,
    char_val: &char,
    state: &mut State,
) {
    match token_type_json_array {
        TokenStage::Opening => json_array_open_case(char_val, state),
        TokenStage::Content(token_stage_content) => {
            token_stage_content_decision(token_stage_content, char_val, state)
        }
        TokenStage::ItemSeparator => json_array_item_separator_case(char_val, state),
        TokenStage::Closing => json_array_closed_case(char_val, state),
    }
}

fn is_val_empty(char_val: &char) -> bool {
    vec![' ', '\t', '\r'].iter().any(|x| x == char_val)
}

fn json_val_open_case_char_empty_val(char_val: &char, state: &mut State) -> bool {
    if !is_val_empty(char_val) {
        return false;
    }
    match state.fields[state.fields.len() - 1].token_type.clone() {
        TokenType::JsonObject(TokenStage::Content(KeyValState::ValState(KeyValType::JsonStr(
            JsonStr::Open(json_str),
        )))) => {
            key_val_json_str_open_case(char_val, state, &json_str);
            true
        }
        TokenType::JsonArray(TokenStage::Content(ArrayValType::JsonStr(JsonStr::Open(
            json_str,
        )))) => {
            array_val_json_str_open_case(char_val, state, &json_str);
            true
        }
        _ => true,
    }
}

fn to_if_req_single(char_val: &char, state: &mut State) {
    if vec!['\n'].iter().any(|x| x == char_val) {
        state.curr_row_num += 1;
        return;
    }

    if state.fields.len() == 0 {
        if is_val_empty(char_val) {
            return;
        }

        let mut field = Field::new();
        if char_val == &'{' {
            field.nesting_state = NestingState::JsonObjectNestinState;
            field.token_type = TokenType::JsonObject(TokenStage::Opening);
        } else if char_val == &'[' {
            field.nesting_state = NestingState::JsonArrayNestingState;
            field.token_type = TokenType::JsonArray(TokenStage::Opening);
        } else {
            unexpected_character_error(char_val, state)
        }

        state.fields.push(field);
        return;
    }

    xml_attributes_check_state(char_val, state);

    if json_val_open_case_char_empty_val(char_val, state) {
        return;
    }

    let token_type = state.fields[state.fields.len() - 1]
        .clone()
        .token_type
        .clone();
    match token_type {
        TokenType::JsonObject(token_type_json_object) => {
            token_type_json_object_decision(token_type_json_object, char_val, state)
        }
        TokenType::JsonArray(token_type_json_object) => {
            token_type_json_array_decision(token_type_json_object, char_val, state)
        }
    }
}

pub fn to_if_req(json: &String) -> Result<String, String> {
    let mut state = State::new();
    for (_, char_val) in json.chars().enumerate() {
        to_if_req_single(&char_val, &mut state);
    }

    Result::Ok(state.curr_xml)
}
