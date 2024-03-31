use iter_tools::Itertools;
use uuid::Uuid;

use super::{
    models::{Field, NestingState, TokenStage, TokenType},
    xml_attributes::{
        self,
        models::{
            XmlAttribute, XmlAttributeArrayinfo, XmlAttributeNoAttributeInfo,
            XmlAttributeObjectInfo, XmlAttributeState, XmlAttributesArrayStages,
            XmlAttributesBasicInfo, XmlAttributesMapKey, XmlAttributesObjectStages,
            XmlAttributesStages, XmlAttributesType,
        },
    },
};

#[derive(Debug)]
pub struct State {
    pub fields: Vec<Field>,
    pub curr_xml: String,
    pub curr_row_num: i32,
    pub curr_indent: i32,
}

impl State {
    pub fn new() -> Self {
        Self {
            fields: Vec::new(),
            curr_xml: String::new(),
            curr_row_num: 1,
            curr_indent: 0,
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
        let last_field = self.fields[last_indx.clone()].clone();
        match (
            last_field.xml_attribute_info.current_state.clone(),
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
                        attibutes_unique_id: Uuid::new_v4().to_string(),
                    });
                    self.fields[last_indx.clone()]
                        .xml_attribute_info
                        .current_state = new_state;
                }
            }
            _ => (),
        }
    }

    pub fn check_end_xml_attributes(&mut self) {
        let last_indx = self.fields.len() - 1;
        let last_field = self.fields[last_indx.clone()].clone();
        for (_, (_, xml_attr_info_type)) in last_field
            .xml_attribute_info
            .xml_attributes_map
            .iter()
            .enumerate()
        {
            match xml_attr_info_type {
                xml_attributes::models::XmlAttributesType::ArrayTypeAttributes(
                    xml_attributees_array_info,
                ) => {
                    self.check_end_xml_attributes_array_handling(xml_attributees_array_info.clone())
                }
                xml_attributes::models::XmlAttributesType::ObjectAttributes(
                    xml_attributes_object_info,
                ) => self
                    .check_end_xml_attributes_object_handling(xml_attributes_object_info.clone()),
                xml_attributes::models::XmlAttributesType::NoAttribute(keys_info) => {
                    self.check_end_xml_no_attributes_handling(keys_info.clone())
                }
            }
        }
    }

    pub fn check_end_xml_attributes_array_handling(
        &mut self,
        xml_attributes_array_info: XmlAttributeArrayinfo,
    ) {
        let xml_attibutes_vec_str = construct_xml_attributes_str_vec(&xml_attributes_array_info);
        for (i, id) in xml_attributes_array_info.unique_key_ids.iter().enumerate() {
            if xml_attibutes_vec_str.len() != 0 && xml_attibutes_vec_str.len() - 1 >= i {
                self.curr_xml = self.curr_xml.replace(id, xml_attibutes_vec_str[i].as_str())
            } else {
                self.curr_xml = self.curr_xml.replace(id, "")
            }
        }
    }

    pub fn check_end_xml_attributes_object_handling(
        &mut self,
        xml_attributes_object_info: XmlAttributeObjectInfo,
    ) {
        let xml_attibutes_vec_str = construct_xml_attributes_str(&xml_attributes_object_info);

        if xml_attibutes_vec_str.len() != 0 {
            self.curr_xml = self.curr_xml.replace(
                xml_attributes_object_info.unique_key_id.as_str(),
                xml_attibutes_vec_str.as_str(),
            );
            let found_indices = self
                .curr_xml
                .match_indices(xml_attributes_object_info.object_id.as_str());
            let found_indices_ints = found_indices
                .into_iter()
                .map(|(indx, _)| indx)
                .collect::<Vec<usize>>();
            let first_part = self.curr_xml.drain(..found_indices_ints[0]);
            let second_part = self
                .curr_xml
                .drain(..found_indices_ints[1] + xml_attributes_object_info.object_id.len());
        } else {
            self.curr_xml = self
                .curr_xml
                .replace(xml_attributes_object_info.unique_key_id.as_str(), "");
            self.curr_xml = self
                .curr_xml
                .replace(xml_attributes_object_info.object_id.as_str(), "");
        }
    }

    pub fn check_end_xml_no_attributes_handling(&mut self, keys_info: XmlAttributeNoAttributeInfo) {
        for (_, id) in keys_info.unique_key_ids.iter().enumerate() {
            self.curr_xml = self.curr_xml.replace(id, "")
        }
    }

    pub fn update_state(
        &mut self,
        basic_info: &XmlAttributesBasicInfo,
        curr_stage: XmlAttributesStages,
    ) {
        let last_index = self.fields.len() - 1;
        self.fields[last_index.clone()]
            .xml_attribute_info
            .current_state = XmlAttributeState::Attributes(XmlAttributesBasicInfo {
            curr_stage,
            current_key: basic_info.current_key.clone(),
            attibutes_unique_id: basic_info.attibutes_unique_id.clone(),
        });
    }

    pub fn abort_xml_attributes(&mut self) {
        let last_index = self.fields.len() - 1;
        self.fields[last_index.clone()]
            .xml_attribute_info
            .current_state = XmlAttributeState::NoAttributes;
    }

    pub fn update_xml_attribute_key(&mut self, xml_atrribute_key: &String) {
        let last_index = self.fields.len() - 1;
        let nesting_state = self.fields[last_index.clone()].nesting_state.clone();
        let key = if let Some(some_key) = self.fields[last_index.clone()].key.clone() {
            some_key
        } else {
            String::new()
        };
        let map_key = XmlAttributesMapKey {
            attribute_type: nesting_state,
            attribute_base_name: key,
        };
        match self.fields[last_index.clone()]
            .xml_attribute_info
            .xml_attributes_map
            .get_mut(&map_key)
        {
            Some(xml_attributes_info) => match xml_attributes_info {
                XmlAttributesType::ArrayTypeAttributes(array_type_info) => {
                    let mut new_attr_vec = array_type_info.attributes.last().unwrap().clone();

                    new_attr_vec.push(XmlAttribute {
                        xml_attribute_value: String::new(),
                        xml_atrribute_key: xml_atrribute_key.clone(),
                    });

                    array_type_info.attributes.pop();
                    array_type_info.attributes.push(new_attr_vec.clone());
                }
                XmlAttributesType::ObjectAttributes(object_type_info) => {
                    object_type_info.attributes.push(XmlAttribute {
                        xml_attribute_value: String::new(),
                        xml_atrribute_key: xml_atrribute_key.clone(),
                    });
                }
                XmlAttributesType::NoAttribute(_) => (),
            },
            None => (),
        }
    }

    pub fn update_xml_attribute_value(&mut self, xml_atrribute_value: &String) {
        let last_index = self.fields.len() - 1;
        let nesting_state = self.fields[last_index.clone()].nesting_state.clone();
        let key = if let Some(some_key) = self.fields[last_index.clone()].key.clone() {
            some_key
        } else {
            String::new()
        };
        let map_key = XmlAttributesMapKey {
            attribute_type: nesting_state,
            attribute_base_name: key,
        };
        match self.fields[last_index.clone()]
            .xml_attribute_info
            .xml_attributes_map
            .get_mut(&map_key)
        {
            Some(xml_attributes_info) => match xml_attributes_info {
                XmlAttributesType::ArrayTypeAttributes(array_type_info) => {
                    let mut new_attr_vec = array_type_info.attributes.last().unwrap().clone();

                    let mut last_attr_info = new_attr_vec.last().unwrap().clone();
                    last_attr_info.xml_attribute_value = xml_atrribute_value.clone();

                    new_attr_vec.pop();
                    new_attr_vec.push(last_attr_info.clone());

                    array_type_info.attributes.pop();
                    array_type_info.attributes.push(new_attr_vec.clone());
                }
                XmlAttributesType::ObjectAttributes(object_type_info) => {
                    let mut last_attribute = object_type_info.attributes.last().unwrap().clone();
                    last_attribute.xml_attribute_value = xml_atrribute_value.clone();

                    object_type_info.attributes.pop();
                    object_type_info.attributes.push(last_attribute.clone());
                }
                XmlAttributesType::NoAttribute(_) => (),
            },
            None => todo!(),
        }
    }
}

fn construct_xml_attributes_str_vec(xml_attributes_info: &XmlAttributeArrayinfo) -> Vec<String> {
    xml_attributes_info
        .attributes
        .iter()
        .map(|attr_vec| {
            attr_vec
                .iter()
                .map(|attr| {
                    format!(
                        "{}=\"{}\"",
                        attr.xml_atrribute_key.clone(),
                        attr.xml_attribute_value.clone()
                    )
                })
                .join(" ")
        })
        .collect::<Vec<String>>()
}

fn construct_xml_attributes_str(xml_attributes_info: &XmlAttributeObjectInfo) -> String {
    xml_attributes_info
        .attributes
        .iter()
        .map(|attr| {
            format!(
                "{}=\"{}\"",
                attr.xml_atrribute_key.clone(),
                attr.xml_attribute_value.clone()
            )
        })
        .join(" ")
}
