use self::stored_procedure_variable::no_stored_procedure_stage;

mod stored_procedure_variable;

#[derive(Clone, Debug)]
pub struct StoredProcedureInfo {
    pub procedure_name: String,
}

#[derive(Clone, Debug)]
pub enum ParameterDirection {
    Input,
    Output,
}

#[derive(Clone, Debug)]
pub enum OracleDbType {
    Varchar2,
    RefCursor,
}

#[derive(Clone, Debug)]
pub struct StoredProcedureParameter {
    pub param_name: String,
    pub param_value: String,
    pub param_direction: ParameterDirection,
    pub param_type: OracleDbType,
    pub position: usize,
}

#[derive(Clone, Debug)]
pub struct StoredProcedure {
    pub info: StoredProcedureInfo,
    pub params: Vec<StoredProcedureParameter>,
}

impl StoredProcedure {
    pub fn new() -> Self {
        StoredProcedure {
            info: StoredProcedureInfo {
                procedure_name: String::new(),
            },
            params: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ProcVariableStages {
    VariableName(String),
    VariableDirection(String),
    VariableType(String),
}

#[derive(Debug, Clone)]
pub enum ProcDecalarationStage {
    NoStoredProcedure,
    ProcedureKeyWord,
    ProcedureName(String),
    OpenBracket,
    Variable(ProcVariableStages),
    VariableSeparator,
    CloseBracket,
}

#[derive(Debug)]
pub struct State {
    pub meta_data_json_arr: String,
    pub request_json_arr: String,
    pub stored_procedures: Vec<StoredProcedure>,
    pub curr_stage: ProcDecalarationStage,
    pub content: Vec<char>,
}

impl State {
    pub fn new() -> Self {
        Self {
            meta_data_json_arr: String::new(),
            request_json_arr: String::new(),
            stored_procedures: Vec::new(),
            content: Vec::new(),
            curr_stage: ProcDecalarationStage::NoStoredProcedure,
        }
    }

    pub fn update_stage(&mut self, stage: &ProcDecalarationStage) {
        self.curr_stage = stage.clone();
    }
}

fn is_white_space(index: usize, state: &State) -> bool {
    vec![' ', '\n', '\t', '\r']
        .iter()
        .any(|x| x == &state.content[index])
}

fn should_not_ignore_white_space(index: usize, state: &mut State) -> bool {
    if !is_white_space(index, state) {
        return false;
    }

    true
}

fn to_json(index: usize, state: &mut State) {
    if should_not_ignore_white_space(index, state) {
        return;
    }

    match state.curr_stage {
        ProcDecalarationStage::NoStoredProcedure => no_stored_procedure_stage(state, index),
        ProcDecalarationStage::ProcedureKeyWord => todo!(),
        ProcDecalarationStage::ProcedureName(_) => todo!(),
        ProcDecalarationStage::OpenBracket => todo!(),
        ProcDecalarationStage::Variable(_) => todo!(),
        ProcDecalarationStage::VariableSeparator => todo!(),
        ProcDecalarationStage::CloseBracket => todo!(),
    }
}

pub fn stored_procedure_to_json(cntnt: &String) -> Result<String, String> {
    let mut state = State::new();
    state.content = cntnt.chars().collect();
    for (index, _) in cntnt.chars().enumerate() {
        to_json(index, &mut state);
    }

    Result::Ok(String::new())
}
