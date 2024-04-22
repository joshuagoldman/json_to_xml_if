use self::{
    json_data_construction::construct_json_data,
    stored_proc_variable::{
        db_type_separator_stage, variable_separator_direction, variable_separator_name,
        variable_separator_new_var, variable_stage_param_direction,
        variable_stage_param_ref_cursor, variable_stage_param_type_in,
        variable_stage_variable_name,
    },
    stored_procedure_variable::{
        no_stored_procedure_stage, open_bracket_stage, stored_procedure_key_word_stage,
        stored_procedure_name_stage,
    },
};

mod json_data_construction;
mod stored_proc_variable;
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

impl StoredProcedureParameter {
    pub fn new() -> Self {
        Self {
            param_name: String::new(),
            param_value: String::new(),
            param_direction: ParameterDirection::Input,
            param_type: OracleDbType::Varchar2,
            position: 0,
        }
    }
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
pub enum ParamSearchType {
    Word,
    EndsWith,
}

#[derive(Debug, Clone)]
pub struct ParamTypeInfo {
    search_type: ParamSearchType,
    str_val: String,
}

#[derive(Debug, Clone)]
pub enum ProcVariableStages {
    VariableName(String),
    VariableDirection(String),
    VariableType(ParamTypeInfo),
}

#[derive(Debug, Clone)]
pub enum VariableSeparationStage {
    NameSeparator,
    InOutSeparator,
    DbTypeSeparator,
    NewVariable,
}

#[derive(Debug, Clone)]
pub enum ProcDecalarationStage {
    NoStoredProcedure,
    ProcedureKeyWord,
    ProcedureName(String),
    OpenBracket,
    Variable(ProcVariableStages),
    VariableSeparator(VariableSeparationStage),
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

    pub fn add_procedure_name(&mut self, str_val: &String) {
        if self.stored_procedures.len() == 0 {
            let mut stored_proc = StoredProcedure::new();
            stored_proc.info.procedure_name = str_val.clone();
            self.stored_procedures.push(stored_proc);
        }
    }

    pub fn update_param_name(&mut self, str_val: &String) {
        let last_proc_index = self.stored_procedures.len() - 1;

        let mut new_param = StoredProcedureParameter::new();
        new_param.param_name = str_val.clone();
        self.stored_procedures[last_proc_index]
            .params
            .push(new_param);
    }

    pub fn update_param_direction(&mut self, param_dircn: &ParameterDirection) {
        let last_proc_index = self.stored_procedures.len() - 1;

        self.stored_procedures[last_proc_index].params[last_proc_index].param_direction =
            param_dircn.clone();
    }

    pub fn update_param_type(&mut self, param_type: &OracleDbType) {
        let last_proc_index = self.stored_procedures.len() - 1;

        self.stored_procedures[last_proc_index].params[last_proc_index].param_type =
            param_type.clone();
    }
}

fn is_white_space(index: &mut usize, state: &State) -> bool {
    vec![' ', '\n', '\t', '\r']
        .iter()
        .any(|x| x == &state.content[index.clone()])
}

fn should_not_ignore_white_space(index: &mut usize, state: &mut State) -> bool {
    if !is_white_space(index, state) {
        return false;
    }

    if let ProcDecalarationStage::Variable(ProcVariableStages::VariableName(curr_proc_name)) =
        state.curr_stage.clone()
    {
        variable_stage_variable_name(state, index, &curr_proc_name)
    }

    true
}

fn param_type_decision(index: &mut usize, state: &mut State, param_type_info: ParamTypeInfo) {
    match param_type_info.search_type {
        ParamSearchType::Word => {
            variable_stage_param_type_in(state, index, &param_type_info.str_val)
        }
        ParamSearchType::EndsWith => {
            variable_stage_param_ref_cursor(state, index, &param_type_info.str_val)
        }
    }
}

fn variable_stages(index: &mut usize, state: &mut State, variable_stage: &ProcVariableStages) {
    match variable_stage {
        ProcVariableStages::VariableName(str_val) => {
            variable_stage_variable_name(state, index, &str_val)
        }
        ProcVariableStages::VariableDirection(str_val) => {
            variable_stage_param_direction(state, index, str_val)
        }
        ProcVariableStages::VariableType(param_type_info) => {
            param_type_decision(index, state, param_type_info.clone())
        }
    }
}

fn variable_separation_stages_decision(
    index: &mut usize,
    state: &mut State,
    var_sep_stages: VariableSeparationStage,
) {
    match var_sep_stages {
        VariableSeparationStage::NameSeparator => variable_separator_name(state, index),
        VariableSeparationStage::InOutSeparator => variable_separator_direction(state, index),
        VariableSeparationStage::DbTypeSeparator => db_type_separator_stage(state, index),
        VariableSeparationStage::NewVariable => variable_separator_new_var(state, index),
    }
}

fn to_json(index: &mut usize, state: &mut State) {
    if should_not_ignore_white_space(index, state) {
        *index = index.clone() + 1;
        return;
    }

    match state.curr_stage.clone() {
        ProcDecalarationStage::NoStoredProcedure => no_stored_procedure_stage(state, &mut *index),
        ProcDecalarationStage::ProcedureKeyWord => stored_procedure_key_word_stage(state, index),
        ProcDecalarationStage::ProcedureName(curr_proc_name) => {
            stored_procedure_name_stage(state, index, &curr_proc_name)
        }
        ProcDecalarationStage::OpenBracket => open_bracket_stage(state, index),
        ProcDecalarationStage::Variable(variable_stage) => {
            variable_stages(index, state, &variable_stage)
        }
        ProcDecalarationStage::VariableSeparator(var_sep_stages) => {
            variable_separation_stages_decision(index, state, var_sep_stages)
        }
    }

    *index = index.clone() + 1;
}

pub fn stored_procedure_to_json(cntnt: &String) -> Result<String, String> {
    let mut state = State::new();
    state.content = cntnt.chars().collect();
    let mut curr_index = 0;
    while curr_index < cntnt.len() {
        to_json(&mut curr_index, &mut state);
    }

    let res = construct_json_data(state.stored_procedures);
    Result::Ok(res)
}
