use super::{
    add_close_tag, add_open_tag,
    models::{KeyValState, TokenStage, TokenStageKey, TokenType},
    unexpected_character_error, Field, NestingState, State,
};

pub fn json_object_open_case(char_val: &char, state: &mut State) {
    match char_val {
        '"' => {
            add_open_tag(state, true);
            state.update_nesting_state(NestingState::JsonObjectNestinState);

            state.fields.push(Field::new());
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::KeyState(TokenStageKey::Opening),
            )));
        }
        '}' => {
            add_open_tag(state, true);
            add_close_tag(state, true);
            state.fields.pop();

            state.update_token_type(TokenType::JsonObject(TokenStage::Closing));
        }
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn json_object_closed_case(char_val: &char, state: &mut State) {
    match char_val {
        ']' => {
            state.fields.pop();

            state.update_to_closed_state()
        }
        '}' => {
            state.fields.pop();

            state.update_to_closed_state();
        }
        ',' => {
            state.update_to_item_separate_state();
        }
        _ => unexpected_character_error(char_val, state),
    }
}

pub fn json_object_item_separator_case(char_val: &char, state: &mut State) {
    match char_val {
        '"' => {
            state.fields.push(Field::new());
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::KeyState(TokenStageKey::Opening),
            )));
        }
        _ => unexpected_character_error(char_val, state),
    }
}
