use std::collections::HashMap;

use super::xml_attributes::models::{XmlAttributesMapKey, XmlAttributesType};

#[derive(Clone, Debug)]
pub struct Field {
    pub token_type: TokenType,
    pub key: Option<String>,
    pub nesting_state: NestingState,
    pub xml_attributes_map_id: String,
}

impl Field {
    pub fn new(map: &mut HashMap<String, HashMap<XmlAttributesMapKey, XmlAttributesType>>) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        map.insert(id.clone(), HashMap::new());
        Self {
            token_type: TokenType::JsonObject(TokenStage::Opening),
            key: None,
            nesting_state: NestingState::JsonObjectNestinState,
            xml_attributes_map_id: id,
        }
    }
}

#[derive(Clone, Debug)]
pub enum JsonStr {
    Open(String),
    Closing,
}

#[derive(Clone, Debug)]
pub enum JsonNull {
    Open(String),
    Closing,
}

#[derive(Clone, Debug)]
pub enum KeyValType {
    JsonStr(JsonStr),
    JsonNumber(String),
    Null(JsonNull),
}

#[derive(Clone, Debug)]
pub enum ArrayValType {
    JsonStr(JsonStr),
    JsonNumber(String),
    Null(JsonNull),
}

#[derive(Clone, Debug)]
pub enum TokenStage<T> {
    Opening,
    Content(T),
    ItemSeparator,
    Closing,
}

#[derive(Clone, Debug)]
pub enum TokenStageKey {
    Opening,
    KeyValSeparator,
    Closing,
}

#[derive(Clone, Debug)]
pub enum KeyValState {
    KeyState(TokenStageKey),
    ValState(KeyValType),
}

#[derive(Clone, Debug)]
pub enum TokenType {
    JsonObject(TokenStage<KeyValState>),
    JsonArray(TokenStage<ArrayValType>),
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum NestingState {
    JsonObjectNestinState,
    JsonArrayNestingState,
}

#[derive(Clone, Debug)]
pub enum XmlOpenTagOptions {
    ArraySimpleVal,
    ObjectSimpleVal,
    ObjectInObject,
    ObjectInArray,
}

#[derive(Clone, Debug)]
pub struct FieldPositionNumForMap {
    pub xml_attr_map_num: usize,
    pub xml_attr_type_num: usize,
}
