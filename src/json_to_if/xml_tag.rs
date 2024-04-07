use super::{
    models::{FieldPositionNumForMap, NestingState, XmlOpenTagOptions},
    state::State,
    xml_attributes::xml_attributes_marking::get_attributes_mark,
};

fn tag_options_attr_decision(
    state: &mut State,
    indentation_str: &String,
    key: &String,
    positions_for_attr_map: FieldPositionNumForMap,
) {
    let default_xml_tag = format!("{}{}<{}>", state.curr_xml, indentation_str, key);
    if let Some(object_id) = state.check_init_xml_attributes(positions_for_attr_map.clone()) {
        state.curr_xml = format!(
            "{}{}{}<{}>",
            state.curr_xml, indentation_str, object_id, key
        );
    } else if let Some(attr_id) = get_attributes_mark(state, key, positions_for_attr_map) {
        state.curr_xml = format!("{}{}<{} {}>", state.curr_xml, indentation_str, key, attr_id);
    } else {
        state.curr_xml = default_xml_tag;
    }
}

pub fn check_if_nested_in_array(state: &mut State) -> XmlOpenTagOptions {
    if state.fields.len() < 2 {
        return XmlOpenTagOptions::ObjectInObject;
    }

    let index = state.fields.len() - 2;
    if state.fields[index].nesting_state == NestingState::JsonArrayNestingState {
        return XmlOpenTagOptions::ObjectInArray;
    }
    XmlOpenTagOptions::ObjectInObject
}

pub fn add_open_tag(state: &mut State, indent: bool, tag_options: XmlOpenTagOptions) {
    state.curr_indent += 1;
    let key = if state.fields.len() == 1 {
        "parameters".to_string()
    } else {
        state.fields[state.fields.len() - 1]
            .key
            .clone()
            .unwrap_or("unknown".to_string())
    };
    let indentation_str = if indent {
        state.get_indentation_str()
    } else {
        "".to_string()
    };

    match tag_options {
        XmlOpenTagOptions::ArraySimpleVal => {
            if let Some(attr_id) = get_attributes_mark(
                state,
                &key,
                FieldPositionNumForMap {
                    xml_attr_type_num: 2,
                    xml_attr_map_num: 3,
                },
            ) {
                state.curr_xml =
                    format!("{}{}<{} {}>", state.curr_xml, indentation_str, key, attr_id);
            } else {
                state.curr_xml = format!("{}{}<{}>", state.curr_xml, indentation_str, key);
            }
        }
        XmlOpenTagOptions::ObjectInArray => {
            tag_options_attr_decision(
                state,
                &indentation_str,
                &key,
                FieldPositionNumForMap {
                    xml_attr_type_num: 2,
                    xml_attr_map_num: 3,
                },
            );
        }
        _ => tag_options_attr_decision(
            state,
            &indentation_str,
            &key,
            FieldPositionNumForMap {
                xml_attr_type_num: 1,
                xml_attr_map_num: 2,
            },
        ),
    }
}

pub fn add_close_tag(state: &mut State, indent: bool) {
    let key = if state.fields.len() == 1 {
        "parameters".to_string()
    } else {
        state.fields[state.fields.len() - 1]
            .key
            .clone()
            .unwrap_or("unknown".to_string())
    };
    let indentation_str = if indent {
        state.get_indentation_str()
    } else {
        "".to_string()
    };

    let default_xml_tag = format!("{}{}</{}>", state.curr_xml, indentation_str, key);
    if let Some(obj_id) = state.get_obj_id_for_closing_tag(&key) {
        state.curr_xml = format!("{}{}</{}>{}", state.curr_xml, indentation_str, key, obj_id);
    } else {
        state.curr_xml = default_xml_tag;
    }
    state.curr_indent -= 1;
}

pub fn add_tag_val(state: &mut State, str_val: &String) {
    if str_val.is_empty() {
        state.curr_xml = format!("{}null", state.curr_xml);
    } else {
        state.curr_xml = format!("{}{}", state.curr_xml, str_val);
    }
}
