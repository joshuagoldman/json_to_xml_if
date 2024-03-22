use crate::json_to_if::TokenStage;

use super::{
    add_close_tag, add_tag_val, unexpected_character_error, ArrayValType, JsonNull, JsonStr, State,
    TokenType,
};

pub fn array_val_json_str_open_case(char_val: &char, state: &mut State, json_str: &String) {
    match char_val {
        '"' => {
            add_tag_val(state, json_str);
            state.fields.pop();

            add_close_tag(state, false);
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
