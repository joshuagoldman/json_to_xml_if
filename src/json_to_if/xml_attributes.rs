use super::State;

pub enum XmlAttributeKeyValueStages {
    Open(String),
    Closed,
}

pub enum XmlAttributesObjectStages {
    Init,
    Key(XmlAttributeKeyValueStages),
    KeyValSeparator,
    Value(XmlAttributeKeyValueStages),
    KeyValFieldSeparator,
    End,
}

pub enum XmlAttributesArrayStages {
    Init,
    ObjectInit,
    Key(XmlAttributeKeyValueStages),
    KeyValSeparator,
    Value(XmlAttributeKeyValueStages),
    KeyValFieldSeparator,
    ObjectEnd,
    ObjectSeparator,
}

pub enum XmlAttributesStages {
    Array,
    Object,
}

pub fn array_attributes_stage_init(char_val: &char, state: &mut State, curr_str_val: &String) {
    let new_str_val = format!("{}{}", curr_str_val, char_val);
    match char_val {
        '{' => {}
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
