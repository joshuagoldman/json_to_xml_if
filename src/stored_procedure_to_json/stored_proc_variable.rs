use crate::IS_ALLOWED_KEY_REGEX_EXPR;

use super::{is_white_space, ProcDecalarationStage, ProcVariableStages, State};

pub fn variable_stage_variable_name(
    state: &mut State,
    index: &mut usize,
    curr_param_name: &String,
) {
    let char_val = state.content[index.clone()];
    let new_str_val = format!("{}{}", curr_param_name, char_val);
    match is_white_space(index, state) {
        true => {
            state.update_param_name(&curr_param_name);
            state.update_stage(&ProcDecalarationStage::VariableSeparator(
                super::VariableSeparationStage::NameSeparator,
            ))
        }
        false => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableName(new_str_val),
        )),
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
        _ => state.abort_param(),
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
    } else if "OUT".contains(&new_param_dir_val.to_uppercase()) {
        state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection(new_param_dir_val),
        ));
    } else {
        state.abort_param();
    }
}

pub fn variable_separator_direction(state: &mut State, index: &mut usize) {
    let allowed_init_vals = vec!['V', 'N', 'B'];
    let char_val = state.content[index.clone()];
    let is_allowed_val = IS_ALLOWED_KEY_REGEX_EXPR
        .get()
        .unwrap()
        .is_match(&char_val.to_string());

    if allowed_init_vals
        .iter()
        .any(|aiv| aiv.to_string() == char_val.to_uppercase().to_string())
    {
        let new_var_stage = super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::Word,
            str_val: char_val.to_string(),
        });
        state.update_stage(&ProcDecalarationStage::Variable(new_var_stage));
    } else if is_allowed_val {
        let new_var_stage = super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::EndsWith,
            str_val: char_val.to_string(),
        });
        state.update_stage(&ProcDecalarationStage::Variable(new_var_stage));
    } else {
        state.abort_param()
    }
}

pub fn variable_stage_param_type_in(state: &mut State, index: &mut usize, param_type_val: &String) {
    let allowed_types = vec!["VARCHAR", "NUMBER", "BOOL"];
    let char_val = state.content[index.clone()];
    let new_param_type_val = format!("{}{}", param_type_val, char_val);
    let index_plus_one = index.clone() + 1;

    if allowed_types
        .iter()
        .any(|at| &new_param_type_val.to_uppercase() == at)
    {
        if state.content.len() > index_plus_one
            && format!("{}{}", new_param_type_val, state.content[index_plus_one]).to_uppercase()
                == "VARCHAR2"
        {
            *index = index_plus_one;
        }
        state.update_param_type(&super::OracleDbType::Varchar2);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::DbTypeSeparator,
        ));
    } else if allowed_types
        .iter()
        .any(|at| at.contains(&new_param_type_val.to_uppercase()))
    {
        let new_var_stage = super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::Word,
            str_val: new_param_type_val,
        });
        state.update_stage(&ProcDecalarationStage::Variable(new_var_stage));
    } else {
        let new_var_stage = super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::EndsWith,
            str_val: new_param_type_val,
        });
        state.update_stage(&ProcDecalarationStage::Variable(new_var_stage));
    }
}

pub fn variable_stage_param_ref_cursor(
    state: &mut State,
    index: &mut usize,
    param_type_val: &String,
) {
    let char_val = state.content[index.clone()];
    let new_param_type_val = format!("{}{}", param_type_val, char_val);
    if char_val == ')' || char_val == ',' || new_param_type_val.len() > 50 {
        state.abort_param()
    } else if new_param_type_val.to_uppercase().ends_with("REFCURSOR") {
        state.update_param_type(&super::OracleDbType::RefCursor);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::DbTypeSeparator,
        ));
    } else if new_param_type_val.to_uppercase().ends_with("%TYPE") {
        state.update_param_type(&super::OracleDbType::Varchar2);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::DbTypeSeparator,
        ));
    } else {
        let new_stage = super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::EndsWith,
            str_val: new_param_type_val,
        });
        state.update_stage(&ProcDecalarationStage::Variable(new_stage));
    }
}

pub fn db_type_separator_stage(state: &mut State, index: &mut usize) {
    let char_val = state.content[index.clone()];

    match char_val {
        ',' => state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::NewVariable,
        )),
        ')' => state.update_stage(&ProcDecalarationStage::NoStoredProcedure),
        _ => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::DefaultKeyWord,
        )),
    }
}

pub fn variable_separator_new_var(state: &mut State, index: &mut usize) {
    let char_val = state.content[index.clone()];
    let is_allowed_val = IS_ALLOWED_KEY_REGEX_EXPR
        .get()
        .unwrap()
        .is_match(&char_val.to_string());
    match is_allowed_val {
        true => state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableName(char_val.to_string()),
        )),
        _ => state.abort_param(),
    }
}

pub fn default_keywrd_stage(state: &mut State, index: &mut usize) {
    let key_word = "DEFAULT ";
    let index_new = index.clone() + key_word.len() - 1;
    let range = index.clone() - 1..index_new;

    if state.content.len() - 1 >= index_new
        && state.content[range]
            .iter()
            .collect::<String>()
            .to_uppercase()
            == key_word
    {
        *index = index_new - 1;
        state.update_stage(&ProcDecalarationStage::Variable(
            ProcVariableStages::DefaultValueKeyWord(String::new()),
        ));
    } else {
        state.abort_param();
    }
}

pub fn default_keyword_value_stage(state: &mut State, index: &mut usize, curr_str_val: &String) {
    let char_val = state.content[index.clone()];
    let new_str_val = format!("{}{}", curr_str_val, char_val);
    let is_allowed_val = IS_ALLOWED_KEY_REGEX_EXPR
        .get()
        .unwrap()
        .is_match(&char_val.to_string());

    if curr_str_val.len() == 0 && is_allowed_val {
        state.update_stage(&ProcDecalarationStage::Variable(
            ProcVariableStages::DefaultValueKeyWord(new_str_val),
        ));
    } else if curr_str_val.len() == 0 || new_str_val.len() > 25 {
        state.abort_param()
    } else if ',' == char_val {
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::NewVariable,
        ))
    } else if char_val == ')' {
        state.update_stage(&ProcDecalarationStage::NoStoredProcedure)
    } else {
        state.update_stage(&ProcDecalarationStage::Variable(
            ProcVariableStages::DefaultValueKeyWord(new_str_val),
        ));
    }
}
