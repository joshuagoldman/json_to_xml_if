use crate::json_to_if::{add_open_tag, Field, JsonNull};

use super::{
    add_close_tag, unexpected_character_error, ArrayValType, JsonStr, NestingState, State,
    TokenStage, TokenType,
};

pub fn json_array_open_case(char_val: &char, state: &mut State) {
    const RADIX: u32 = 10;
    match char_val {
        '"' => {
            state.update_nesting_state(NestingState::JsonArrayNestingState);
            add_open_tag(state, true);

            let mut field = Field::new();
            field.key = state.fields[state.fields.len() - 1].key.clone();
            state.fields.push(field);
            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonStr(JsonStr::Open(String::new())),
            )));
        }
        '{' => {
            state.update_nesting_state(NestingState::JsonArrayNestingState);
            let mut field = Field::new();
            field.key = state.fields[state.fields.len() - 1].key.clone();
            state.fields.push(field);

            state.update_token_type(TokenType::JsonObject(TokenStage::Opening));
        }
        ']' => {
            state.fields.pop();
            state.update_token_type(TokenType::JsonArray(TokenStage::Closing));
        }
        'n' => {
            state.update_nesting_state(NestingState::JsonArrayNestingState);
            add_open_tag(state, true);
            state.fields.push(Field::new());

            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::Null(JsonNull::Open("n".to_string())),
            )));
        }
        _ => match char_val.to_digit(RADIX) {
            Some(_) => {
                state.update_nesting_state(NestingState::JsonArrayNestingState);
                add_open_tag(state, true);
                state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                    ArrayValType::JsonNumber(String::new()),
                )));
            }
            None => unexpected_character_error(char_val, state),
        },
    }
}

pub fn json_array_closed_case(char_val: &char, state: &mut State) {
    match char_val {
        '}' => {
            add_close_tag(state, true);
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
    const RADIX: u32 = 10;
    match char_val {
        '"' => {
            add_open_tag(state, true);

            let mut field = Field::new();
            field.key = state.fields[state.fields.len() - 1].key.clone();
            state.fields.push(field);
            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::JsonStr(JsonStr::Open(String::new())),
            )));
        }
        '{' => {
            let mut field = Field::new();
            field.key = state.fields[state.fields.len() - 1].key.clone();
            state.fields.push(field);

            state.update_token_type(TokenType::JsonObject(TokenStage::Opening));
        }
        'n' => {
            add_open_tag(state, true);

            let mut field = Field::new();
            field.key = state.fields[state.fields.len() - 1].key.clone();
            state.fields.push(field);
            state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                ArrayValType::Null(JsonNull::Open("n".to_string())),
            )));
        }
        _ => match char_val.to_digit(RADIX) {
            Some(_) => {
                add_open_tag(state, true);
                let mut field = Field::new();
                field.key = state.fields[state.fields.len() - 1].key.clone();
                state.fields.push(field);

                state.update_token_type(TokenType::JsonArray(TokenStage::Content(
                    ArrayValType::JsonNumber(char_val.to_string()),
                )));
            }
            None => unexpected_character_error(char_val, state),
        },
    }
}
