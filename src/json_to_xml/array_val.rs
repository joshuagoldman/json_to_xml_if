use super::{
    models::{ArrayValType, JsonNull, JsonStr, TokenStage, TokenType},
    state::State,
    unexpected_character_error,
    xml_tag::{add_close_tag, add_close_tag_val_empty, add_tag_val},
};

fn array_close_value_handling(state: &mut State, str_val: &String) {
    add_tag_val(state, str_val);
    if str_val.is_empty() {
        add_close_tag_val_empty(state);
    } else {
        add_close_tag(state, false);
    }
    state.fields.pop();
}

pub fn array_val_json_str_open_case(char_val: &char, state: &mut State, json_str: &String) {
    match char_val {
        '"' => {
            array_close_value_handling(state, json_str);

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonStr(JsonStr::Closing),
            )));
        }
        _ => {
            let new_str = format!("{}{}", json_str, char_val);
            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonStr(JsonStr::Open(new_str)),
            )));
        }
    }
}

pub fn array_val_json_str_close_case(char_val: &char, state: &mut State) {
    match char_val {
        ',' => state.update_to_item_separate_state(),
        ']' => {
            state.fields.pop();
            state.update_to_closed_state();
        }
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn array_val_json_number_open_case(
    char_val: &char,
    state: &mut State,
    json_num_as_str: &String,
) {
    let new_num_as_str = format!("{}{}", json_num_as_str, char_val);
    match char_val {
        ',' => {
            array_close_value_handling(state, json_num_as_str);

            state.update_to_item_separate_state();
        }
        ']' => {
            array_close_value_handling(state, json_num_as_str);

            state.update_to_closed_state();
        }
        _ => match new_num_as_str.parse::<i16>() {
            Ok(_) => {
                state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                    ArrayValType::JsonNumber(new_num_as_str),
                )));
            }
            _ => unexpected_character_error(char_val, state),
        },
    }
}

pub fn array_val_json_null_case_open(char_val: &char, state: &mut State, curr_str_val: &String) {
    let new_str_val = format!("{}{}", curr_str_val, char_val);
    match new_str_val == "null" {
        true => {
            array_close_value_handling(state, &String::new());

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::Null(JsonNull::Closing),
            )));
        }
        _ => match "null".contains(new_str_val.as_str()) {
            true => {
                state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                    ArrayValType::Null(JsonNull::Open(new_str_val)),
                )));
            }
            false => unexpected_character_error(char_val, state),
        },
    }
}

pub fn array_val_json_null_case_closed(char_val: &char, state: &mut State) {
    match char_val {
        ',' => state.update_to_item_separate_state(),
        ']' => {
            state.fields.pop();
            state.update_to_closed_state();
        }
        _ => unexpected_character_error(char_val, state),
    }
}
