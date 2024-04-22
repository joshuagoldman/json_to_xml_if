use iter_tools::Itertools;

use super::{ParameterDirection, StoredProcedure, StoredProcedureParameter};

//pub struct StoredProcedureParameter {
//    pub param_name: String,
//    pub param_value: String,
//    pub param_direction: ParameterDirection,
//    pub param_type: OracleDbType,
//    pub position: usize,
//}
fn construct_json_meta_data_param_decision(stored_proc_param: &StoredProcedureParameter) -> String {
    let indentation_str = "    ".to_string();
    match stored_proc_param.param_type {
        super::OracleDbType::Varchar2 => {
            format!("{}{{\n{} \"paramName\": {},\n{} \"paramValue\": \"\",\n{} \"paramDirection\": \"Input\",\n{} \"paramType\": \"Varchar2\"",
            indentation_str, indentation_str, stored_proc_param.param_name, indentation_str, indentation_str, indentation_str)
        }
        super::OracleDbType::RefCursor => {
            format!("{}{{\n{} \"paramName\": {},\n{} \"paramValue\": \"\",\n{} \"paramDirection\": \"Output\",\n{} \"paramType\": \"Refcursor\"",
            indentation_str, indentation_str, stored_proc_param.param_name, indentation_str, indentation_str, indentation_str)
        }
    }
}

fn construct_json_meta_data(stored_proc: &StoredProcedure) -> String {
    let params = stored_proc
        .params
        .iter()
        .map(|p| construct_json_meta_data_param_decision(p))
        .collect::<String>();

    format!("   {{\n{}\n}}", params)
}

fn construct_json_for_class(stored_proc: &StoredProcedure) -> String {
    let in_params = stored_proc
        .params
        .iter()
        .filter(|p| match p.param_direction {
            ParameterDirection::Input => true,
            ParameterDirection::Output => false,
        })
        .map(|p| format!("    \"{}\": \"\"", p.param_name))
        .join("\n,");

    format!("   {{\n{}\n   }}", in_params)
}

pub fn construct_json_data(stored_procedures: Vec<StoredProcedure>) -> String {
    let mut array_cntnt = stored_procedures
        .iter()
        .map(|sp| {
            let meta_data = construct_json_meta_data(sp);
            let json_class_data = construct_json_for_class(sp);

            format!(
                " {{\n  \"metaData\": {{\n{}\n  }},\n  \"class\": {{\n{}\n  }}\n }}",
                meta_data, json_class_data
            )
        })
        .collect::<String>();
    array_cntnt = format!("[\n{}\n]", array_cntnt);
    array_cntnt
}
