use convert_case::{Case, Casing};
use iter_tools::Itertools;

use super::{ParameterDirection, StoredProcedure, StoredProcedureParameter};

fn construct_json_meta_data_param_decision(stored_proc_param: &StoredProcedureParameter) -> String {
    let indentation_str = "    ".to_string();
    let (param_direction, param_type) = match (stored_proc_param.param_type.clone(),stored_proc_param.param_direction.clone()) {
        (super::OracleDbType::RefCursor, ParameterDirection::Output) => ("Output", "Refcursor"),
        (super::OracleDbType::RefCursor, ParameterDirection::Input) => ("Input", "Refcursor"),
        (super::OracleDbType::Varchar2, ParameterDirection::Input) => ("Input", "Varchar2"),
        (super::OracleDbType::Varchar2, ParameterDirection::Output) => ("Output", "Varchar2")
    };
    format!("{}{{\n{} \"paramName\": \"{}\",\n{} \"paramValue\": \"\",\n{} \"paramDirection\": \"{}\",\n{} \"paramType\": \"{}\",\n{} \"position\": \"{}\"\n{}}}",
    indentation_str, 
    indentation_str,
    stored_proc_param.param_name,
    indentation_str,
    indentation_str, 
    param_direction,
    indentation_str, 
    param_type,
    indentation_str, 
    stored_proc_param.position,
    indentation_str)
}

fn construct_json_meta_data(stored_proc: &StoredProcedure) -> String {
    println!("{:#?}", stored_proc);
    let params = stored_proc
        .params
        .iter()
        .map(|p| construct_json_meta_data_param_decision(p))
        .join(",\n");

    let indentation_str = "   ";
    let info_obj = format!("{}\"info\": {{\n{} \"procedureName\": \"{}\"\n{}}}",
                           indentation_str, 
                           indentation_str,
                           stored_proc.info.procedure_name.to_case(Case::Camel),
                           indentation_str);
    format!("{},\n{}\"params\": [\n{}\n{}]", info_obj,indentation_str, params, indentation_str)
}

fn construct_json_for_class(stored_proc: &StoredProcedure) -> String {
    let indentation_str = "   ";
    let in_params = stored_proc
        .params
        .iter()
        .filter(|p| match p.param_direction {
            ParameterDirection::Input => true,
            ParameterDirection::Output => false,
        })
        .map(|p| format!("{} \"{}\": \"\"", indentation_str, p.param_name.to_case(Case::Camel)))
        .join(",\n");

    in_params
}

pub fn construct_json_data(stored_procedures: Vec<StoredProcedure>) -> String {
    //println!("{:#?}", stored_procedures);
    
    let indentation_str = " ";
    let mut array_cntnt = stored_procedures
        .iter()
        .map(|sp| {
            let meta_data = construct_json_meta_data(sp);
            let json_class_data = construct_json_for_class(sp);

            format!(
                "{}{{\n{} \"metaData\": {{\n{}\n{} }},\n{} \"class\": {{\n{}\n{} }}\n{}}}",
                indentation_str,
                indentation_str,
                meta_data,
                indentation_str,
                indentation_str,
                json_class_data,
                indentation_str,
                indentation_str,
            )
        })
        .join(",\n");
    array_cntnt = format!("[\n{}\n]", array_cntnt);
    array_cntnt
}
