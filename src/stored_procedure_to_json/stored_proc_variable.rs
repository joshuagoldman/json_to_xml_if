use crate::IS_ALLOWED_KEY_REGEX_EXPR;

use super::{is_white_space, ProcDecalarationStage, State};

pub fn variable_stage_variable_name(state: &mut State, index: &mut usize, curr_proc_name: &String) {
    let char_val = state.content[index.clone()];
    let new_str_val = format!("{}{}", curr_proc_name, char_val);
    match is_white_space(index.clone(), state) {
        true => {
            state.update_param_name(&curr_proc_name);
            state.update_stage(&ProcDecalarationStage::VariableSeparator(
                super::VariableSeparationStage::NameSeparator,
            ))
        }
        false => {
            let is_allowed_proc_name = IS_ALLOWED_KEY_REGEX_EXPR
                .get()
                .unwrap()
                .is_match(&new_str_val);
            match is_allowed_proc_name {
                true => state.update_stage(&ProcDecalarationStage::Variable(
                    super::ProcVariableStages::VariableName(new_str_val),
                )),
                false => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
            }
        }
    }
}

pub fn variable_separator_name(state: &mut State, index: &mut usize) {
    let char_val = state.content[index.clone()];
    match char_val.to_uppercase().to_string().as_str() {
        "I" => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection("I".to_string()),
        )),
        "O" => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection("O".to_string()),
        )),
        _ => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
    }
}

pub fn variable_stage_param_direction(
    state: &mut State,
    index: &mut usize,
    param_dir_val: &String,
) {
    let char_val = state.content[index.clone()];
    let new_param_dir_val = format!("{}{}", param_dir_val, char_val);
    if new_param_dir_val.to_uppercase() == "IN" {
        state.update_param_direction(&super::ParameterDirection::Input);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::InOutSeparator,
        ));
    } else if new_param_dir_val.to_uppercase() == "OUT" {
        state.update_param_direction(&super::ParameterDirection::Output);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::InOutSeparator,
        ));
    } else if "OUT".contains(&new_param_dir_val) {
        super::ProcVariableStages::VariableDirection(new_param_dir_val);
    } else {
        state.update_stage(&ProcDecalarationStage::NoStoredProcedure);
    }
}

pub fn variable_separator_direction(state: &mut State, index: &mut usize) {
    let char_val = state.content[index.clone()];
    match char_val.to_uppercase().to_string().as_str() {
        "V" => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection("V".to_string()),
        )),
        "R" => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection("R".to_string()),
        )),
        _ => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
    }
}

pub fn variable_stage_param_type_varchar(
    state: &mut State,
    index: &mut usize,
    param_type_val: &String,
) {
    let char_val = state.content[index.clone()];
    let new_param_type_val = format!("{}{}", param_type_val, char_val);
    if char_val == ')' || char_val == ',' {
        state.update_stage(&ProcDecalarationStage::NoStoredProcedure);
    } else if new_param_type_val.to_uppercase() == "VARCHAR2" {
        state.update_param_type(&super::OracleDbType::Varchar2);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::DbTypeSeparator,
        ));
    } else if "VARCHAR2".contains(&new_param_type_val) {
        super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::Word,
            str_val: new_param_type_val,
        });
    } else {
        super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::EndsWith,
            str_val: new_param_type_val,
        });
    }
}

pub fn variable_stage_param_ref_cursor(
    state: &mut State,
    index: &mut usize,
    param_type_val: &String,
) {
    let char_val = state.content[index.clone()];
    let new_param_type_val = format!("{}{}", param_type_val, char_val);
    if char_val == ')' || char_val == ',' {
        state.update_stage(&ProcDecalarationStage::NoStoredProcedure);
    } else if new_param_type_val.to_uppercase().ends_with("REFCURSOR") {
        state.update_param_type(&super::OracleDbType::RefCursor);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::DbTypeSeparator,
        ));
    } else {
        super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::EndsWith,
            str_val: new_param_type_val,
        });
    }
}

pub fn db_type_separator_stage(state: &mut State, index: &mut usize) {
    let char_val = state.content[index.clone()];

    match char_val {
        ',' => state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::NameSeparator,
        )),
        _ => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
    }
}
