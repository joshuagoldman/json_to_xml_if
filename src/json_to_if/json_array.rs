use crate::json_to_if::models::JsonNull;

use super::{
    models::{Field, XmlOpenTagOptions},
    unexpected_character_error,
    xml_tag::{add_close_tag, add_open_tag, add_open_tag_val_empty, add_tag_val},
    ArrayValType, JsonStr, NestingState, State, TokenStage, TokenType,
};

fn new_arr_value_handling_empty_val(state: &mut State) {
    new_arr_value_handling_general(state, true)
}

fn new_arr_value_handling(state: &mut State) {
    new_arr_value_handling_general(state, false)
}

fn new_arr_value_handling_general(state: &mut State, is_val_empty: bool) {
    state.update_nesting_state(NestingState::JsonArrayNestingState);
    let mut field = Field::new(&mut state.xml_attributes_map);
    field.key = state.fields[state.fields.len() - 1].key.clone();
    state.fields.push(field);
    if is_val_empty {
        add_open_tag_val_empty(state, XmlOpenTagOptions::ArraySimpleVal);
    } else {
        add_open_tag(state, true, XmlOpenTagOptions::ArraySimpleVal);
    }
}

fn check_null_val_is_attr(state: &mut State) {
    if let Some(_) = state.xml_attributes {
        state.update_nesting_state(NestingState::JsonArrayNestingState);
        let mut field = Field::new(&mut state.xml_attributes_map);
        field.key = state.fields[state.fields.len() - 1].key.clone();
        state.fields.push(field);
        add_open_tag_val_empty(state, XmlOpenTagOptions::ObjectInArray);
    } else {
        new_arr_value_handling_empty_val(state)
    }
}

pub fn json_array_open_case(char_val: &char, state: &mut State) {
    match char_val {
        '"' => {
            new_arr_value_handling(state);
            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonStr(JsonStr::Open(String::new())),
            )));
        }
        '{' => {
            state.update_nesting_state(NestingState::JsonArrayNestingState);
            let mut field = Field::new(&mut state.xml_attributes_map);
            field.key = state.fields[state.fields.len() - 1].key.clone();
            state.fields.push(field);

            state.update_token_type(TokenType::JsonObject(TokenStage::Opening));
        }
        ']' => {
            add_open_tag(state, true, XmlOpenTagOptions::ArraySimpleVal);
            add_tag_val(state, &String::new());
            add_close_tag(state, false);

            state.fields.pop();
            state.update_token_type(TokenType::JsonArray(TokenStage::Closing));
        }
        'n' => {
            check_null_val_is_attr(state);

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::Null(JsonNull::Open("n".to_string())),
            )));
        }
        '-' => {
            new_arr_value_handling(state);

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonNumber(char_val.to_string()),
            )));
        }
        _ => match char_val.to_string().parse::<i32>() {
            Ok(_) => {
                new_arr_value_handling(state);

                state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                    ArrayValType::JsonNumber(char_val.to_string()),
                )));
            }
            _ => unexpected_character_error(char_val, state),
        },
    }
}

pub fn json_array_closed_case(char_val: &char, state: &mut State) {
    match char_val {
        '}' => {
            add_close_tag(state, true);
            state.check_end_xml_attributes();
            state.fields.pop();
            state.update_to_closed_state();
        }
        ',' => {
            state.update_to_item_separate_state();
        }
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn json_array_item_separator_case(char_val: &char, state: &mut State) {
    match char_val {
        '"' => {
            new_arr_value_handling(state);

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonStr(JsonStr::Open(String::new())),
            )));
        }
        '{' => {
            let mut field = Field::new(&mut state.xml_attributes_map);
            field.key = state.fields[state.fields.len() - 1].key.clone();
            state.fields.push(field);

            state.update_token_type(TokenType::JsonObject(TokenStage::Opening));
        }
        'n' => {
            check_null_val_is_attr(state);

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::Null(JsonNull::Open("n".to_string())),
            )));
        }
        '-' => {
            new_arr_value_handling(state);

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonNumber(char_val.to_string()),
            )));
        }
        _ => match char_val.to_string().parse::<i32>() {
            Ok(_) => {
                new_arr_value_handling(state);

                state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                    ArrayValType::JsonNumber(char_val.to_string()),
                )));
            }
            _ => unexpected_character_error(char_val, state),
        },
    }
}
