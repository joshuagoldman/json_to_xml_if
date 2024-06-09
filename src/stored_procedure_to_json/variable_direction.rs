use super::{InOutDirectionStage, InOutDirectionStageInfo, ProcDecalarationStage, State};

pub fn variable_stage_param_direction_init(
    state: &mut State,
    index: &mut usize,
    param_dir_val: &String,
) {
    let char_val = state.content[index.clone()];
    let new_param_dir_val = format!("{}{}", param_dir_val, char_val);

    if new_param_dir_val.to_uppercase() == "IN" {
        state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection(InOutDirectionStageInfo {
                str_val: new_param_dir_val,
                stage: InOutDirectionStage::InOut,
            }),
        ));
    } else if new_param_dir_val.to_uppercase() == "OUT" {
        state.update_param_direction(&super::ParameterDirection::Output);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::InOutSeparator,
        ));
    } else if "OUT".contains(&new_param_dir_val.to_uppercase()) {
        state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection(InOutDirectionStageInfo {
                str_val: new_param_dir_val,
                stage: InOutDirectionStage::Init,
            }),
        ));
    } else {
        state.abort_param();
    }
}

pub fn variable_stage_param_direction_in_out(
    state: &mut State,
    index: &mut usize,
    param_dir_val: &String,
) {
    let char_val = state.content[index.clone()];
    let new_param_dir_val = format!("{}{}", param_dir_val, char_val);

    if new_param_dir_val.to_uppercase().to_string() == "INOUT".to_string() {
        state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection(InOutDirectionStageInfo {
                str_val: new_param_dir_val,
                stage: InOutDirectionStage::ReqEmptySpace,
            }),
        ));
    } else if "INOUT"
        .to_string()
        .contains(new_param_dir_val.to_uppercase().as_str())
    {
        state.update_stage(&ProcDecalarationStage::Variable(
            super::ProcVariableStages::VariableDirection(InOutDirectionStageInfo {
                str_val: new_param_dir_val,
                stage: InOutDirectionStage::InOut,
            }),
        ));
    } else {
        state.update_param_direction(&super::ParameterDirection::Input);
        let new_var_stage = super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::Word,
            str_val: new_param_dir_val.chars().skip(2).collect::<String>(),
        });
        state.update_stage(&ProcDecalarationStage::Variable(new_var_stage));
    }
}

pub fn variable_stage_param_direction_req_empty_space(
    state: &mut State,
    index: &mut usize,
    param_dir_val: &String,
) {
    let char_val = state.content[index.clone()];

    if char_val.to_uppercase().to_string() == " ".to_string() {
        state.update_param_direction(&super::ParameterDirection::InputOutput);
        state.update_stage(&ProcDecalarationStage::VariableSeparator(
            super::VariableSeparationStage::InOutSeparator,
        ));
    } else {
        state.update_param_direction(&super::ParameterDirection::Input);
        let new_var_stage = super::ProcVariableStages::VariableType(super::ParamTypeInfo {
            search_type: super::ParamSearchType::Word,
            str_val: param_dir_val.chars().skip(2).collect::<String>(),
        });
        state.update_stage(&ProcDecalarationStage::Variable(new_var_stage));
    }
}
