use crate::IS_ALLOWED_KEY_REGEX_EXPR;

use super::{is_white_space, State};

pub fn package_name_stage(state: &mut State, index: &mut usize, curr_str_val: &String) {
    let char_val = state.content[index.clone()];
    let new_str_val = format!("{}{}", curr_str_val, char_val);

    let is_allowed_val = IS_ALLOWED_KEY_REGEX_EXPR
        .get()
        .unwrap()
        .is_match(&char_val.to_string());

    if curr_str_val.len() == 0 && is_allowed_val {
        state.update_stage(&super::ProcDecalarationStage::PackageName(new_str_val))
    } else if curr_str_val.len() == 0 {
        state.update_stage(&super::ProcDecalarationStage::NoStoredProcedure);
    } else if is_white_space(index, state) {
        state.update_stage(&super::ProcDecalarationStage::NoStoredProcedure);
        state.update_package_name(&curr_str_val)
    } else {
        state.update_stage(&super::ProcDecalarationStage::PackageName(new_str_val))
    }
}
