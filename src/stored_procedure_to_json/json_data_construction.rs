use super::{ParameterDirection, StoredProcedure};

fn construct_json_for_class(stored_proc: &StoredProcedure) -> String {
    let in_params = stored_proc
        .params
        .iter()
        .filter(|p| match p.param_direction {
            ParameterDirection::Input => true,
            ParameterDirection::Output => false,
        })
        .map(|p| format!(" \"{}\": \"\"", p.param_name))
        .collect::<String>();

    format!("{{]\n{}\n}}", in_params)
}

fn construct_json_meta_data(stored_proc: &StoredProcedure) -> String {
    let in_params = stored_proc
        .params
        .iter()
        .filter(|p| match p.param_direction {
            ParameterDirection::Input => true,
            ParameterDirection::Output => false,
        })
        .map(|p| format!(" \"{}\": \"\"", p.param_name))
        .collect::<String>();

    format!("   {{\n{}\n   }}", in_params)
}

pub fn construct_json_data(stored_procedures: Vec<StoredProcedure>) -> String {
    let mut array_cntnt = stored_procedures
        .iter()
        .map(|sp| {
            let meta_data = construct_json_meta_data(sp);
            let json_class_data = construct_json_for_class(sp);

            format!(
                " {{\n  \"metaData\": {{\n  {}\n  }},\n  \"class\": {{\n  {}\n  }}\n }}",
                meta_data, json_class_data
            )
        })
        .collect::<String>();
    array_cntnt = format!("[\n{}\n]", array_cntnt);
    array_cntnt
}
