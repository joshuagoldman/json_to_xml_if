use std::collections::HashMap;

use crate::json_to_if::NestingState;

#[derive(Debug, Clone)]
pub struct XmlAttribute {
    pub xml_atrribute_key: String,
    pub xml_attribute_value: String,
}

#[derive(Debug, Clone)]
pub struct XmlAttributeObjectInfo {
    pub attributes: Vec<XmlAttribute>,
    pub unique_key_id: String,
    pub object_id: String,
}

#[derive(Debug, Clone)]
pub struct XmlAttributeArrayinfo {
    pub attributes: Vec<Vec<XmlAttribute>>,
    pub unique_key_ids: Vec<String>,
    pub object_id: String,
}

#[derive(Debug, Clone)]
pub struct XmlAttributeNoAttributeInfo {
    pub unique_key_ids: Vec<String>,
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
    NullValue(String),
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
    NullValue(String),
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct XmlAttributesMapKey {
    pub attribute_base_name: String,
    pub attribute_type: NestingState,
}

#[derive(Debug, Clone)]
pub struct XmlAttributesBasicInfo {
    pub current_key: XmlAttributesMapKey,
    pub curr_stage: XmlAttributesStages,
}

#[derive(Debug, Clone)]
pub enum XmlAttributeState {
    NoAttributes,
    Attributes(XmlAttributesBasicInfo),
}

#[derive(Debug, Clone)]
pub struct XmlAttributesInfo {
    pub xml_attributes_map: HashMap<XmlAttributesMapKey, XmlAttributesType>,
    pub current_state: XmlAttributeState,
}

#[derive(Debug, Clone)]
pub struct XmlAttributesUniqIds {
    pub attr_id: String,
    pub attr_object_id: String,
}
