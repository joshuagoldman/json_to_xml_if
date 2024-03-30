use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct XmlAttribute {
    xml_atrribute_key: String,
    xml_attribute_value: String,
}

#[derive(Debug, Clone)]
pub struct XmlAttributeObjectInfo {
    attributes: Vec<XmlAttribute>,
}

#[derive(Debug, Clone)]
pub struct XmlAttributeArrayinfo {
    attributes: Vec<Vec<XmlAttribute>>,
    keys_amount: i32,
}

#[derive(Debug, Clone)]
pub struct XmlAttributeNoAttributeInfo {
    keys_amount: i32,
}

#[derive(Debug, Clone)]
pub enum XmlAttributeKeyValueStages {
    Open(String),
    Closed,
}

#[derive(Debug, Clone)]
pub enum XmlAttributesObjectStages {
    Init,
    Key(XmlAttributeKeyValueStages),
    KeyValSeparator,
    Value(XmlAttributeKeyValueStages),
    KeyValFieldSeparator,
}

#[derive(Debug, Clone)]
pub enum XmlAttributesArrayStages {
    Init,
    ObjectInit,
    Key(XmlAttributeKeyValueStages),
    KeyValSeparator,
    Value(XmlAttributeKeyValueStages),
    KeyValFieldSeparator,
    ObjectEnd,
    ObjectSeparator,
}

#[derive(Debug, Clone)]
pub enum XmlAttributesStages {
    Array(XmlAttributesArrayStages),
    Object(XmlAttributesObjectStages),
}

#[derive(Debug, Clone)]
pub enum XmlAttributesType {
    ArrayTypeAttributes(XmlAttributeArrayinfo),
    ObjectAttributes(XmlAttributeObjectInfo),
    NoAttribute(XmlAttributeNoAttributeInfo),
}

#[derive(Debug, Clone)]
pub struct XmlAttributesBasicInfo {
    pub current_key: String,
    pub curr_stage: XmlAttributesStages,
}

#[derive(Debug, Clone)]
pub enum XmlAttributeState {
    NoAttributes,
    Attributes(XmlAttributesBasicInfo),
}

#[derive(Debug, Clone)]
pub struct XmlAttributesInfo {
    pub xml_attributes_map: HashMap<String, XmlAttributesType>,
    pub current_state: XmlAttributeState,
}

impl XmlAttributesInfo {
    pub fn update_state(&mut self, current_key: String, curr_stage: XmlAttributesStages) {
        self.current_state = XmlAttributeState::Attributes(XmlAttributesBasicInfo {
            curr_stage,
            current_key,
        });
    }

    pub fn abort_xml_attributes(&mut self) {
        self.current_state = XmlAttributeState::NoAttributes;
    }

    pub fn update_xml_attribute_key(&mut self, current_key: &String, xml_atrribute_key: &String) {
        match self.xml_attributes_map.get_mut(current_key) {
            Some(xml_attributes_info) => match xml_attributes_info {
                XmlAttributesType::ArrayTypeAttributes(array_type_info) => {
                    let new_attr_vec = array_type_info.attributes.last().unwrap();

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
            None => todo!(),
        }
    }

    pub fn update_xml_attribute_value(
        &mut self,
        current_key: &String,
        xml_atrribute_value: &String,
    ) {
        match self.xml_attributes_map.get_mut(current_key) {
            Some(xml_attributes_info) => match xml_attributes_info {
                XmlAttributesType::ArrayTypeAttributes(array_type_info) => {
                    let new_attr_vec = array_type_info.attributes.last().unwrap();

                    let mut last_attr_info = new_attr_vec.last().unwrap();
                    last_attr_info.xml_attribute_value = xml_atrribute_value.clone();

                    new_attr_vec.pop();
                    new_attr_vec.push(last_attr_info.clone());

                    array_type_info.attributes.pop();
                    array_type_info.attributes.push(new_attr_vec.clone());
                }
                XmlAttributesType::ObjectAttributes(object_type_info) => {
                    let last_attribute = object_type_info.attributes.last().unwrap();
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
