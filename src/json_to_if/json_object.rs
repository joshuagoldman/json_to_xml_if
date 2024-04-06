use super::{
    models::{KeyValState, TokenStage, TokenStageKey, TokenType, XmlOpenTagOptions},
    unexpected_character_error,
    xml_tag::{add_close_tag, add_open_tag, check_if_nested_in_array},
    Field, State,
};

pub fn json_object_open_case(char_val: &char, state: &mut State) {
    match char_val {
        '"' => {
            let options = check_if_nested_in_array(state);
            add_open_tag(state, true, options);

            state.fields.push(Field::new(&mut state.xml_attributes_map));
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::KeyState(TokenStageKey::Opening),
            )));
        }
        '}' => {
            add_open_tag(state, true, XmlOpenTagOptions::ObjectOpening);
            add_close_tag(state, false);
            state.check_end_xml_attributes();
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
            state.fields.push(Field::new(&mut state.xml_attributes_map));
            state.update_token_type(TokenType::JsonObject(TokenStage::Content(
                KeyValState::KeyState(TokenStageKey::Opening),
            )));
        }
        _ => unexpected_character_error(char_val, state),
    }
}
