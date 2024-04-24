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

fn construct_json_meta_data(stored_proc: &StoredProcedure, package_name: &String) -> String {
    let params = stored_proc
        .params
        .iter()
        .map(|p| construct_json_meta_data_param_decision(p))
        .join(",\n");

    let procedure_name_full: String;

    if !package_name.is_empty() {
        procedure_name_full = format!("{}.{}", package_name, stored_proc.info.procedure_name)
    }
    else {
        procedure_name_full = stored_proc.info.procedure_name.clone();
    }

    let indentation_str = "   ";
    let info_obj = format!("{}\"info\": {{\n{} \"procedureName\": \"{}\"\n{}}}",
                           indentation_str, 
                           indentation_str,
                            procedure_name_full,
                           indentation_str);
    format!("{},\n{}\"params\": [\n{}\n{}]", info_obj,indentation_str, params, indentation_str)
}

fn modify_param_name(str_val: &String) -> String {
    let start_var_vals = vec![
        "PI",
        "I",
        "P_IN"
    ];
    let mut new_str_val = str_val.to_uppercase().clone();
    
    for (_,start_var_val) in start_var_vals.iter().enumerate() {
        new_str_val = new_str_val.replacen(format!("{}_", start_var_val).as_str(), "", 1);
    }
    new_str_val.to_case(Case::Camel)
}

fn construct_json_for_class(stored_proc: &StoredProcedure) -> String {

    let indentation_str = "   ";
    let mut in_params = stored_proc
        .params
        .iter()
        .filter(|p| match p.param_direction {
            ParameterDirection::Input => true,
            ParameterDirection::Output => false,
        })
        .map(|x| x.param_name.clone())
        .collect::<Vec<String>>();

    if in_params.iter().all(|x| x.to_uppercase().starts_with("I")) {
        in_params = in_params.iter().map(|x| x.chars().skip(1).collect::<String>())
        .collect::<Vec<String>>();

    }

    let in_params_str = in_params
        .iter()
        .map(|p| format!("{} \"{}\": \"\"", indentation_str, modify_param_name(&p)))
        .join(",\n");

    in_params_str
}

pub fn construct_json_data(stored_procedures: Vec<StoredProcedure>, package_name: &String) -> String {
    //println!("{:#?}", stored_procedures);
    
    let indentation_str = " ";
    let mut array_cntnt = stored_procedures
        .iter()
        .map(|sp| {
            let meta_data = construct_json_meta_data(sp, package_name);
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
