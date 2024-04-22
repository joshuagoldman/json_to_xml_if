use std::i16;

use crate::IS_ALLOWED_KEY_REGEX_EXPR;

use super::{ProcDecalarationStage, State};

pub fn no_stored_procedure_stage(state: &mut State, index: &mut usize) {
    let key_word = "PROCEDURE ";
    let index_new = index.clone() + key_word.len();
    let range = index.clone()..index_new;
    if state.content.len() <= index.clone() + key_word.len()
        && state.content[range].iter().collect::<String>() == key_word
    {
        *index = index_new;
        state.update_stage(&ProcDecalarationStage::ProcedureKeyWord);
    }
}

pub fn stored_procedure_key_word_stage(state: &mut State, index: &mut usize) {
    let char_val = state.content[index.clone()];
    match char_val.to_string().parse::<i16>() {
        Ok(_) => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
        Err(_) => {
            state.update_stage(&ProcDecalarationStage::ProcedureName(char_val.to_string()));
        }
    }
}

pub fn stored_procedure_name_stage(state: &mut State, index: &mut usize, curr_proc_name: &String) {
    let char_val = state.content[index.clone()];
    let new_str_val = format!("{}{}", curr_proc_name, char_val);
    match char_val {
        '(' => state.update_stage(&ProcDecalarationStage::OpenBracket),
        _ => {
            let is_allowed_proc_name = IS_ALLOWED_KEY_REGEX_EXPR
                .get()
                .unwrap()
                .is_match(&new_str_val);
            match is_allowed_proc_name {
                true => state.update_stage(&ProcDecalarationStage::ProcedureName(new_str_val)),
                false => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
            }
        }
    }
}

pub fn open_bracket_stage(state: &mut State, index: &mut usize) {
    let char_val = state.content[index.clone()];
    let is_allowed_proc_name = IS_ALLOWED_KEY_REGEX_EXPR
        .get()
        .unwrap()
        .is_match(&char_val.to_string());
    match is_allowed_proc_name {
        true => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableName(char_val.to_string()),
        )),
        false => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
    }
}
