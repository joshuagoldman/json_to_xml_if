use super::{ProcDecalarationStage, State};

pub fn is_comment(state: &mut State, index: &mut usize) -> bool {
    if let ProcDecalarationStage::CommentSection(_) = state.curr_stage.clone() {
        return false;
    }

    let new_index = index.clone() + 1;

    if state.content.len() - 1 >= new_index
        && state.content[index.clone()..new_index + 1]
            .iter()
            .collect::<String>()
            == "/*".to_string()
    {
        state.update_stage(&ProcDecalarationStage::CommentSection(super::CommentInfo {
            comment_type: super::CommentType::Section,
            previous_stage: Box::new(state.curr_stage.clone()),
        }));
        *index = new_index;
        return true;
    } else if state.content.len() - 1 >= new_index
        && state.content[index.clone()..new_index + 1]
            .iter()
            .collect::<String>()
            == "--".to_string()
    {
        state.update_stage(&ProcDecalarationStage::CommentSection(super::CommentInfo {
            comment_type: super::CommentType::OneLiner,
            previous_stage: Box::new(state.curr_stage.clone()),
        }));
        *index = new_index;
        return true;
    }
    return false;
}

pub fn comment_type_one_liner(
    state: &mut State,
    index: &mut usize,
    previous_stage: &ProcDecalarationStage,
) {
    let char_val = state.content[index.clone()];
    match char_val {
        '\n' => {
            state.update_stage(previous_stage);
        }
        _ => (),
    }
}

pub fn comment_type_section(
    state: &mut State,
    index: &mut usize,
    previous_stage: &ProcDecalarationStage,
) {
    let new_index = index.clone() + 1;

    if state.content.len() - 1 >= new_index
        && state.content[index.clone()..new_index + 1]
            .iter()
            .collect::<String>()
            == "*/".to_string()
    {
        state.update_stage(previous_stage);
        *index = new_index;
    }
}
