use std::collections::HashMap;

use crate::ATTRIBUTES_REGEX_EXPR;

use super::{
    models::{Field, FieldPositionNumForMap, NestingState, TokenStage, TokenType},
    xml_attributes::{
        self,
        models::{
            XmlAttributesBasicInfo, XmlAttributesMapKey, XmlAttributesStages, XmlAttributesType,
        },
        xml_attributes_abort::abort_attributes,
        xml_attributes_end::{
            check_end_xml_attributes_array_handling, check_end_xml_attributes_object_handling,
            check_end_xml_no_attributes_handling,
        },
        xml_attributes_init::init_xml_attributes,
        xml_attributes_object_id::get_attributes_object_id_for_closing_tag,
        xml_attributes_update::{
            update_xml_attribute_arr_index, update_xml_attribute_key, update_xml_attribute_value,
        },
    },
};

#[derive(Debug)]
pub struct State {
    pub fields: Vec<Field>,
    pub curr_xml: String,
    pub curr_row_num: i32,
    pub curr_indent: i32,
    pub xml_attributes: Option<XmlAttributesBasicInfo>,
    pub xml_attributes_map: HashMap<String, HashMap<XmlAttributesMapKey, XmlAttributesType>>,
    pub to_snake_case: bool,
    pub root_name: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            curr_xml: String::new(),
            curr_row_num: 1,
            curr_indent: 0,
            xml_attributes: None,
            xml_attributes_map: HashMap::new(),
            to_snake_case: true,
            root_name: String::new(),
        }
    }

    pub fn update_to_item_separate_state(&mut self) {
        if let NestingState::JsonArrayNestingState =
            self.fields[self.fields.len() - 1].nesting_state
        {
            let len = self.fields.len() - 1;
            self.fields[len.clone()].token_type = TokenType::JsonArray(TokenStage::ItemSeparator);
        } else if let NestingState::JsonObjectNestinState =
            self.fields[self.fields.len() - 1].nesting_state
        {
            let len = self.fields.len() - 1;
            self.fields[len.clone()].token_type = TokenType::JsonObject(TokenStage::ItemSeparator);
        }
    }

    pub fn update_to_closed_state(&mut self) {
        if self.fields.len() < 1 {
            return;
        }

        match self.fields[self.fields.len() - 1].nesting_state {
            NestingState::JsonObjectNestinState => {
                self.update_token_type(TokenType::JsonArray(TokenStage::Closing))
            }
            NestingState::JsonArrayNestingState => {
                self.update_token_type(TokenType::JsonObject(TokenStage::Closing))
            }
        }
    }

    pub fn update_token_type(&mut self, token_type: TokenType) {
        let len = self.fields.len() - 1;
        self.fields[len.clone()].token_type = token_type;
    }

    pub fn update_nesting_state(&mut self, nesting_state: NestingState) {
        let len = self.fields.len() - 1;
        self.fields[len.clone()].nesting_state = nesting_state;
    }

    pub fn get_indentation_str(&mut self) -> String {
        let mut tabs_as_str = String::new();
        for _ in 0..self.curr_indent {
            tabs_as_str.push(' ');
        }
        tabs_as_str.pop();

        format!("\n{}", tabs_as_str)
    }

    pub fn check_init_xml_attributes(
        &mut self,
        field_index_for_map: FieldPositionNumForMap,
    ) -> Option<String> {
        init_xml_attributes(self, field_index_for_map)
    }

    pub fn get_obj_id_for_closing_tag(&mut self, key: &String) -> Option<String> {
        match self.xml_attributes.clone() {
            Some(xml_attr_info) => {
                let xml_key_base = ATTRIBUTES_REGEX_EXPR
                    .get()
                    .unwrap()
                    .replace(key.as_str(), "");

                if xml_attr_info.current_key.attribute_base_name == xml_key_base {
                    get_attributes_object_id_for_closing_tag(self, &xml_attr_info)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn check_end_xml_attributes(&mut self) {
        let last_indx = self.fields.len() - 1;
        let last_field = self.fields[last_indx].clone();
        match self
            .xml_attributes_map
            .clone()
            .get(&last_field.xml_attributes_map_id)
        {
            Some(att_map) => {
                for (_, (_, xml_attr_info_type)) in att_map.iter().enumerate() {
                    match xml_attr_info_type {
                        xml_attributes::models::XmlAttributesType::ArrayTypeAttributes(
                            xml_attributees_array_info,
                        ) => check_end_xml_attributes_array_handling(
                            self,
                            xml_attributees_array_info.clone(),
                        ),
                        xml_attributes::models::XmlAttributesType::ObjectAttributes(
                            xml_attributes_object_info,
                        ) => check_end_xml_attributes_object_handling(
                            self,
                            xml_attributes_object_info.clone(),
                        ),
                        xml_attributes::models::XmlAttributesType::NoAttribute(keys_info) => {
                            check_end_xml_no_attributes_handling(self, keys_info.clone())
                        }
                    }
                }
            }
            None => (),
        }
        self.xml_attributes_map
            .remove(&last_field.xml_attributes_map_id);
    }

    pub fn update_state(&mut self, curr_stage: XmlAttributesStages) {
        if let Some(xml_attribtues_basic_info) = self.xml_attributes.clone() {
            self.xml_attributes = Some(XmlAttributesBasicInfo {
                curr_stage,
                current_key: xml_attribtues_basic_info.current_key,
                attr_id: xml_attribtues_basic_info.attr_id,
            })
        }
    }

    pub fn abort_xml_attributes(&mut self) {
        if let Some(xml_attr_basic_info) = self.xml_attributes.clone() {
            abort_attributes(self, &xml_attr_basic_info);
            self.xml_attributes = None;
        }
    }

    pub fn update_xml_attribute_key(&mut self, xml_atrribute_key: &String) {
        if let Some(basic_info) = self.xml_attributes.clone() {
            update_xml_attribute_key(self, &basic_info, xml_atrribute_key)
        }
    }

    pub fn update_xml_attribute_value(&mut self, xml_atrribute_value: &String) {
        if let Some(basic_info) = self.xml_attributes.clone() {
            update_xml_attribute_value(self, &basic_info, xml_atrribute_value)
        }
    }

    pub fn update_xml_attribute_arr_index(&mut self) {
        if let Some(basic_info) = self.xml_attributes.clone() {
            update_xml_attribute_arr_index(self, &basic_info)
        }
    }
}
