use super::{ProcDecalarationStage, State};

pub fn no_stored_procedure_stage(state: &mut State, index: usize) {
    let key_word = "PROCEDURE ";
    if state.content.len() <= index + key_word.len()
        && state.content[index..index + key_word.len()]
            .iter()
            .collect::<String>()
            == key_word
    {
        state.update_stage(&ProcDecalarationStage::ProcedureKeyWord);
    }
}
