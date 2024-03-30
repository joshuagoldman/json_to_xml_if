use std::collections::HashMap;

use super::{
    models::{Field, NestingState, TokenStage, TokenType},
    xml_attributes::{
        self,
        models::{
            XmlAttributeArrayinfo, XmlAttributeState, XmlAttributesArrayStages,
            XmlAttributesBasicInfo, XmlAttributesInfo, XmlAttributesMapKey,
            XmlAttributesObjectStages, XmlAttributesStages,
        },
    },
};

#[derive(Debug)]
pub struct State {
    pub fields: Vec<Field>,
    pub curr_xml: String,
    pub curr_row_num: i32,
    pub curr_indent: i32,
    pub xml_attribute_info: XmlAttributesInfo,
}

impl State {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            curr_xml: String::new(),
            curr_row_num: 1,
            curr_indent: 0,
            xml_attribute_info: XmlAttributesInfo {
                xml_attributes_map: HashMap::new(),
                current_state: XmlAttributeState::NoAttributes,
            },
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

    pub fn check_init_xml_attributes(&mut self) {
        let last_indx = self.fields.len() - 1;
        let last_field = self.fields[last_indx.clone()];
        match (
            self.xml_attribute_info.current_state,
            last_field.key.clone(),
        ) {
            (XmlAttributeState::NoAttributes, Some(key)) => {
                if key.to_uppercase().ends_with("_ATTRIBUTES") {
                    let map_key = XmlAttributesMapKey {
                        attribute_type: last_field.nesting_state.clone(),
                        attribute_base_name: key,
                    };

                    let curr_stage =
                        if let NestingState::JsonObjectNestinState = last_field.nesting_state {
                            XmlAttributesStages::Object(XmlAttributesObjectStages::Init)
                        } else {
                            XmlAttributesStages::Array(XmlAttributesArrayStages::Init)
                        };
                    let new_state = XmlAttributeState::Attributes(XmlAttributesBasicInfo {
                        current_key: map_key,
                        curr_stage,
                    });
                    self.xml_attribute_info.current_state = new_state;
                }
            }
            _ => (),
        }
    }

    pub fn check_end_xml_attributes(&mut self) {
        match self.xml_attribute_info.current_state.clone() {
            XmlAttributeState::Attributes(attr_basic_info) => {
                if let Some(xml_attr_info_type) = self
                    .xml_attribute_info
                    .xml_attributes_map
                    .get(&attr_basic_info.current_key)
                {
                    match xml_attr_info_type {
                        xml_attributes::models::XmlAttributesType::ArrayTypeAttributes(_) => {
                            todo!()
                        }
                        xml_attributes::models::XmlAttributesType::ObjectAttributes(_) => todo!(),
                        xml_attributes::models::XmlAttributesType::NoAttribute(_) => todo!(),
                    }
                }
            }
            XmlAttributeState::NoAttributes => (),
        }
    }

    pub fn check_end_xml_attributes_array_handling(
        &mut self,
        xml_attributes_array_info: XmlAttributeArrayinfo,
    ) {
        xml_attributes_array_info.xml_attributes_array_info
    }
}
