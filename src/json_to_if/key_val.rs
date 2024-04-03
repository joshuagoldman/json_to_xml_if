use regex::Regex;

use crate::json_to_if::{
    add_open_tag,
    models::{JsonNull, JsonStr, KeyValState, KeyValType},
};

use super::{
    add_close_tag, add_tag_val,
    models::{TokenStage, TokenStageKey, TokenType},
    unexpected_character_error, State,
};

fn json_key_update(state: &mut State, char_val: &char) -> String {
    let len = state.fields.len() - 1;
    match state.fields[len.clone()].key.clone() {
        Some(some_key) => {
            let new_key = format!("{}{}", some_key, char_val);
            state.fields[len].key = Some(new_key.clone());
            new_key
        }
        None => {
            state.fields[len].key = Some(char_val.to_string());
            char_val.to_string()
        }
    }
}

pub fn key_open_case(char_val: &char, state: &mut State) {
    let regex = Regex::new(r"^[aA-zZ]").unwrap();
    match char_val {
        '"' => {
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::KeyState(TokenStageKey::Closing),
            )));
        }
        _ => {
            let new_key = json_key_update(state, char_val);
            if let None = regex.captures(new_key.as_str()) {
                panic!("unexpected key name at row {}", state.curr_row_num)
            }
        }
    }
}

pub fn key_closed_cased(char_val: &char, state: &mut State) {
    match char_val {
        ':' => state.update_token_type(TokenType::JsonObject(TokenStage::Content(
            KeyValState::KeyState(TokenStageKey::KeyValSeparator),
        ))),
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn key_val_separator_case(char_val: &char, state: &mut State) {
    const RADIX: u32 = 10;
    match char_val {
        '"' => {
            add_open_tag(state, true);
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::ValState(KeyValType::JsonStr(JsonStr::Open(String::new()))),
            )));
        }
        '{' => {
            state.update_token_type(TokenType::JsonObject(TokenStage::Opening));
        }
        'n' => {
            add_open_tag(state, true);
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::ValState(KeyValType::Null(JsonNull::Open("n".to_string()))),
            )));
        }
        '[' => {
            state.update_token_type(TokenType::JsonArray(TokenStage::Opening));
        }
        _ => match char_val.to_digit(RADIX) {
            Some(_) => {
                add_open_tag(state, true);
                state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                    KeyValState::ValState(KeyValType::JsonNumber(char_val.to_string())),
                )));
            }
            None => unexpected_character_error(char_val, state),
        },
    }
}

pub fn key_val_json_str_open_case(char_val: &char, state: &mut State, json_str: &String) {
    match char_val {
        '"' => {
            add_tag_val(state, json_str);
            add_close_tag(state, false);
            state.fields.pop();
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::ValState(KeyValType::JsonStr(JsonStr::Closing)),
            )));
        }
        _ => {
            let new_str = format!("{}{}", json_str, char_val);
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::ValState(KeyValType::JsonStr(JsonStr::Open(new_str))),
            )));
        }
    }
}

pub fn key_val_json_str_close_case(char_val: &char, state: &mut State) {
    match char_val {
        ',' => {
            state.update_to_item_separate_state();
        }
        '}' => {
            add_close_tag(state, true);
            state.check_end_xml_attributes();
            state.fields.pop();
            state.update_to_closed_state();
        }
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn key_val_json_number_open_case(char_val: &char, state: &mut State, json_num_as_str: &String) {
    let new_num_as_str = format!("{}{}", json_num_as_str, char_val);
    match char_val {
        ',' => {
            add_tag_val(state, json_num_as_str);
            add_close_tag(state, false);
            state.fields.pop();

            state.update_to_item_separate_state();
        }
        '}' => {
            add_tag_val(state, json_num_as_str);
            add_close_tag(state, false);
            state.fields.pop();

            add_close_tag(state, true);
            state.check_end_xml_attributes();
            state.fields.pop();
            state.update_to_closed_state();
        }
        _ => match new_num_as_str.parse::<i32>() {
            Ok(_) => {
                state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                    KeyValState::ValState(KeyValType::JsonNumber(new_num_as_str.to_owned())),
                )));
            }
            _ => unexpected_character_error(char_val, state),
        },
    }
}

pub fn key_val_json_null_case_open(char_val: &char, state: &mut State, curr_str_val: &String) {
    let new_str_val = format!("{}{}", curr_str_val, char_val);
    match new_str_val == "null" {
        true => {
            add_tag_val(state, &"null".to_string());
            add_close_tag(state, false);
            state.fields.pop();

            add_close_tag(state, true);
            state.check_end_xml_attributes();
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::ValState(KeyValType::Null(JsonNull::Closing)),
            )));
        }
        _ => match "null".contains(new_str_val.as_str()) {
            true => {
                state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                    KeyValState::ValState(KeyValType::Null(JsonNull::Open(new_str_val))),
                )));
            }
            false => unexpected_character_error(char_val, state),
        },
    }
}

pub fn key_val_json_null_case_closed(char_val: &char, state: &mut State) {
    match char_val {
        ',' => {
            state.update_to_item_separate_state();
        }
        '}' => {
            state.fields.pop();
            state.update_to_closed_state();
        }
        _ => unexpected_character_error(char_val, state),
    }
}
