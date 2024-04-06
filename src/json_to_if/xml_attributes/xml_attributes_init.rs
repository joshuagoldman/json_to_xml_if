use crate::json_to_if::{
    models::{
        FieldPositionNumForMap, NestingState, TokenType, XmlOpenTagOptions, ATTRIBUTES_REGEX_EXPR,
    },
    state::State,
    xml_tag::check_if_nested_in_array,
};

use super::{
    models::{
        XmlAttributesArrayStages, XmlAttributesBasicInfo, XmlAttributesMapKey,
        XmlAttributesObjectStages, XmlAttributesStages, XmlAttributesUniqIds,
    },
    xml_attributes_object_id::get_attributes_object_id,
};

pub fn init_xml_attributes(
    state: &mut State,
    field_pos_with_rel_map: FieldPositionNumForMap,
) -> Option<XmlAttributesUniqIds> {
    if state.fields.len() < field_pos_with_rel_map.xml_attr_map_num {
        return None;
    }

    let map_index = state.fields.len() - field_pos_with_rel_map.xml_attr_map_num;
    let last_index = state.fields.len() - 1;
    let map_field = state.fields[map_index.clone()].clone();
    let last_field = state.fields[last_index.clone()].clone();

    match last_field.key.clone() {
        Some(key) => {
            if key.to_uppercase().ends_with("_ATTRIBUTES") {
                let xml_key_base = ATTRIBUTES_REGEX_EXPR
                    .get()
                    .unwrap()
                    .replace(key.as_str(), "");
                let nesting_state: NestingState;
                let curr_stage: XmlAttributesStages;
                if let XmlOpenTagOptions::ArrayValOpening = check_if_nested_in_array(state) {
                    curr_stage = XmlAttributesStages::Array(XmlAttributesArrayStages::ObjectInit);
                    nesting_state = NestingState::JsonArrayNestingState;
                } else {
                    curr_stage = XmlAttributesStages::Object(XmlAttributesObjectStages::Init);
                    nesting_state = NestingState::JsonObjectNestinState;
                };
                let map_key = XmlAttributesMapKey {
                    attribute_type: nesting_state,
                    attribute_base_name: xml_key_base.to_string(),
                };

                let basic_info = XmlAttributesBasicInfo {
                    current_key: map_key.clone(),
                    curr_stage,
                    attr_id: map_field.xml_attributes_map_id.clone(),
                };
                state.xml_attributes = Some(basic_info.clone());

                get_attributes_object_id(state, &basic_info)
            } else {
                None
            }
        }
        _ => None,
    }
}
