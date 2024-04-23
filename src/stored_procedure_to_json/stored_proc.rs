use std::i16;

use crate::IS_ALLOWED_KEY_REGEX_EXPR;

use super::{ProcDecalarationStage, State};

pub fn no_stored_procedure_stage(state: &mut State, index: &mut usize) {
    let key_word_package = "PACKAGE ";
    let index_new_package = index.clone() + key_word_package.len();
    let range_package = index.clone()..index_new_package;

    let key_word_procedure = "PROCEDURE ";
    let index_new_procedure = index.clone() + key_word_procedure.len();
    let range_procedure = index.clone()..index_new_procedure;
    if state.content.len() - 1 >= index_new_procedure
        && state.content[range_procedure]
            .iter()
            .collect::<String>()
            .to_uppercase()
            == key_word_procedure
    {
        *index = index_new_procedure - 1;
        state.update_stage(&ProcDecalarationStage::ProcedureKeyWord);
    } else if state.content.len() - 1 >= index_new_package
        && state.content[range_package]
            .iter()
            .collect::<String>()
            .to_uppercase()
            == key_word_package
    {
        *index = index_new_package - 1;
        state.update_stage(&ProcDecalarationStage::PackageName(String::new()));
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
        '(' => {
            state.add_procedure_name(curr_proc_name);
            state.update_stage(&ProcDecalarationStage::OpenBracket);
        }
        _ => {
            let is_allowed_proc_name = IS_ALLOWED_KEY_REGEX_EXPR
                .get()
                .unwrap()
                .is_match(&new_str_val);
            if curr_proc_name.len() > 0 {
                state.update_stage(&ProcDecalarationStage::ProcedureName(new_str_val));
            } else if is_allowed_proc_name {
                state.update_stage(&ProcDecalarationStage::ProcedureName(new_str_val));
            } else {
                state.abort_param()
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
