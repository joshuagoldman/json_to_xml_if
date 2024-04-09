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
    xml_attributes::{
        models::{XmlAttributeKeyValueStages, XmlAttributesArrayStages, XmlAttributesObjectStages},
        xml_attributes_array::array_attributes_stage_value_open,
        xml_attributes_check_state,
        xml_attributes_object::object_attributes_stage_value_open,
    },
};

pub mod array_val;
mod json_array;
mod json_object;
mod key_val;
pub mod models;
pub mod state;
pub mod xml_attributes;
pub mod xml_tag;

fn unexpected_character_error(char_val: &char, state: &State) {
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
    vec![' ', '\n', '\t', '\r'].iter().any(|x| x == char_val)
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
        }
        TokenType::JsonArray(TokenStage::Content(ArrayValType::JsonStr(JsonStr::Open(
            json_str,
        )))) => {
            array_val_json_str_open_case(char_val, state, &json_str);
        }
        _ => (),
    }
    json_val_open_case_char_empty_val_xml_attr(char_val, state);
    true
}

fn json_val_open_case_char_empty_val_xml_attr(char_val: &char, state: &mut State) {
    match state.xml_attributes.clone() {
        Some(xml_attr_basic_info) => match xml_attr_basic_info.curr_stage {
            xml_attributes::models::XmlAttributesStages::Array(
                XmlAttributesArrayStages::Value(XmlAttributeKeyValueStages::Open(curr_val)),
            ) => {
                array_attributes_stage_value_open(char_val, state, &curr_val);
            }
            xml_attributes::models::XmlAttributesStages::Object(
                XmlAttributesObjectStages::Value(XmlAttributeKeyValueStages::Open(curr_val)),
            ) => {
                object_attributes_stage_value_open(char_val, state, &curr_val);
            }
            _ => (),
        },
        _ => (),
    }
}

fn to_if_req_single(char_val: &char, state: &mut State) {
    if vec!['\n'].iter().any(|x| x == char_val) {
        state.curr_row_num += 1;
    }

    if state.fields.len() == 0 {
        if is_val_empty(char_val) {
            return;
        }

        let mut field = Field::new(&mut state.xml_attributes_map);
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
    xml_attributes_check_state(char_val, state);
}

pub fn json_to_xml(
    json: &String,
    to_snake_case: bool,
    root_name: String,
) -> Result<String, String> {
    let mut state = State::new();
    state.to_snake_case = to_snake_case;
    state.root_name = root_name;
    for (_, char_val) in json.chars().enumerate() {
        to_if_req_single(&char_val, &mut state);
    }

    Result::Ok(state.curr_xml.trim().to_string())
}
