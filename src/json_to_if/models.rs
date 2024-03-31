use std::{cell::OnceCell, sync::OnceLock};

use regex::Regex;

pub static IS_ALLOWED_KEY_REGEX_EXPR: OnceLock<Regex> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct Field {
    pub token_type: TokenType,
    pub key: Option<String>,
    pub nesting_state: NestingState,
}

impl Field {
    pub fn new() -> Self {
        Self {
            token_type: TokenType::JsonObject(TokenStage::Opening),
            key: None,
            nesting_state: NestingState::JsonObjectNestinState,
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
