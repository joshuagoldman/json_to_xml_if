use self::{
    cdata::open_tag_stage_cdata_open,
    close_tag::{
        closed_tag_angle_bracket, closed_tag_key_stage, closed_tag_opening,
        closed_tag_sibling_or_closing, closed_tag_value_stage_forward_slash,
    },
    open_tag::{
        closed_key_is_angle_bracket, closed_key_is_empty_value, open_tag_key_stage_open,
        open_tag_value_stage,
    },
    xml_attribute::{
        xml_attribute_key_closed, xml_attribute_key_open, xml_attribute_value_closed,
        xml_attribute_value_open,
    },
};

mod cdata;
mod close_tag;
pub mod json_build;
mod open_tag;
mod xml_attribute;

#[derive(Clone, Debug)]
pub struct Node {
    pub child_node_result: ChildNodesOrKeyVal,
    pub node_key: Option<String>,
    pub xml_attributes: Vec<XmlAttribute>,
    pub stage: NodeStage,
}

impl Node {
    pub fn new() -> Self {
        Self {
            child_node_result: ChildNodesOrKeyVal::KeyValue(NodeStrResult {
                xml_attributes_str: String::new(),
                str_value: String::new(),
                key: String::new(),
                is_object: true,
            }),
            stage: NodeStage::OpenTag(OpenTagStage::Key),
            xml_attributes: Vec::new(),
            node_key: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChildNodesOrKeyVal {
    KeyValue(NodeStrResult),
    ChildNodes(Vec<NodeStrResult>),
}

#[derive(Debug, Clone)]
pub struct XmlAttribute {
    pub attribute_key: String,
    pub attribute_val: String,
}

#[derive(Debug, Clone)]
pub enum ValueStage {
    Open(String),
    Closed,
}

#[derive(Debug, Clone)]
pub enum XmlAttributeStage {
    AttributeKey(ValueStage),
    AttributeValue(ValueStage),
}

#[derive(Debug, Clone)]
pub struct InitEndKeys {
    open_tag_key: String,
    closed_tag_key: String,
}

#[derive(Debug, Clone)]
pub enum OpenTagStage {
    Key,
    Attributes(XmlAttributeStage),
    TagValueCData(String),
    TagValue(String),
    AngelBracket,
    IsEmptyValue,
}

#[derive(Debug, Clone)]
pub enum ClosedTagStage {
    ClosedTagOpening,
    ForwardSlash,
    Key(InitEndKeys),
    AngelBracket,
    SibingOrClosing,
}

#[derive(Debug, Clone)]
pub enum NodeStage {
    OpenTag(OpenTagStage),
    ClosedTag(ClosedTagStage),
}

#[derive(Debug, Clone)]
pub struct NodeStrResult {
    pub xml_attributes_str: String,
    pub str_value: String,
    pub key: String,
    pub is_object: bool,
}

#[derive(Debug)]
pub struct State {
    pub nodes: Vec<Node>,
    pub curr_row_num: i32,
    pub curr_indent: i32,
    pub str_json: Option<String>,
    pub to_camel_case: bool,
}

impl State {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            curr_row_num: 1,
            curr_indent: 0,
            str_json: None,
            to_camel_case: true,
        }
    }

    pub fn update_node_stage(&mut self, node_stage: NodeStage) {
        let len = self.nodes.len() - 1;

        self.nodes[len].stage = node_stage.clone();
    }

    pub fn update_node_key_val(&mut self, node_result: NodeStrResult) {
        let len = self.nodes.len() - 1;
        self.nodes[len].child_node_result = ChildNodesOrKeyVal::KeyValue(node_result);
    }

    pub fn update_node_result_parent(&mut self, node_result: NodeStrResult) {
        let pos = self.nodes.len() - 2;
        match self.nodes[pos].child_node_result.clone() {
            ChildNodesOrKeyVal::ChildNodes(child_nodes) => {
                let mut new_child_nodes = child_nodes.clone();
                new_child_nodes.push(node_result);
                self.nodes[pos].child_node_result = ChildNodesOrKeyVal::ChildNodes(new_child_nodes);
            }
            ChildNodesOrKeyVal::KeyValue(_) => {
                let new_child_nodes = vec![node_result];
                self.nodes[pos].child_node_result = ChildNodesOrKeyVal::ChildNodes(new_child_nodes);
            }
        }
    }

    pub fn update_attribute_key(&mut self, key: &String) {
        let len = self.nodes.len() - 1;

        self.nodes[len].xml_attributes.push(XmlAttribute {
            attribute_key: key.to_owned(),
            attribute_val: String::new(),
        })
    }

    pub fn update_attribute_val(&mut self, str_val: &String) {
        let len = self.nodes.len() - 1;

        if self.nodes[len].xml_attributes.len() == 0 {
            self.nodes[len].xml_attributes.push(XmlAttribute {
                attribute_key: String::new(),
                attribute_val: str_val.clone(),
            })
        } else {
            let len_xml_attr = self.nodes[len].xml_attributes.len() - 1;

            self.nodes[len].xml_attributes[len_xml_attr].attribute_val = str_val.clone();
        }
    }

    pub fn get_indentation_str(&mut self) -> String {
        let mut tabs_as_str = String::new();
        for _ in 0..self.curr_indent {
            tabs_as_str.push(' ');
        }

        tabs_as_str
    }
}

fn xml_attribute_stage_key_decision(
    char_val: &char,
    state: &mut State,
    xml_attribute_stage_key: ValueStage,
) {
    match xml_attribute_stage_key {
        ValueStage::Open(str_val) => xml_attribute_key_open(char_val, state, &str_val),
        ValueStage::Closed => xml_attribute_key_closed(char_val, state),
    }
}

fn xml_attribute_stage_val_decision(
    char_val: &char,
    state: &mut State,
    xml_attribute_stage_value: ValueStage,
) {
    match xml_attribute_stage_value {
        ValueStage::Open(str_val) => xml_attribute_value_open(char_val, state, &str_val),
        ValueStage::Closed => xml_attribute_value_closed(char_val, state, false),
    }
}

fn open_tag_stage_attributes_decision(
    char_val: &char,
    state: &mut State,
    open_tag_stage_attributes: XmlAttributeStage,
) {
    match open_tag_stage_attributes {
        XmlAttributeStage::AttributeKey(xml_attribute_stage_key) => {
            xml_attribute_stage_key_decision(char_val, state, xml_attribute_stage_key)
        }
        XmlAttributeStage::AttributeValue(xml_attribute_stage_val) => {
            xml_attribute_stage_val_decision(char_val, state, xml_attribute_stage_val)
        }
    }
}

fn unexpected_character_error(char_val: &char, state: &State) {
    print!("{:#?}", state);
    panic!(
        "Unexpected character '{}' at row {}",
        char_val, state.curr_row_num
    )
}

fn is_white_space(char_val: &char) -> bool {
    vec![' ', '\n', '\t', '\r'].iter().any(|x| x == char_val)
}

fn should_not_ignore_white_space(char_val: &char, state: &mut State) -> bool {
    if !is_white_space(char_val) {
        return false;
    }
    if let None = state.nodes[state.nodes.len() - 1].node_key {
        return true;
    }

    let last_node = state.nodes[state.nodes.len() - 1].stage.clone();
    match last_node {
        NodeStage::OpenTag(OpenTagStage::Key) => {
            open_tag_key_stage_open(char_val, state, true);
        }
        NodeStage::OpenTag(OpenTagStage::Attributes(XmlAttributeStage::AttributeValue(
            ValueStage::Closed,
        ))) => {
            xml_attribute_value_closed(char_val, state, true);
        }
        NodeStage::OpenTag(OpenTagStage::TagValue(_))
        | NodeStage::OpenTag(OpenTagStage::TagValueCData(_))
        | NodeStage::OpenTag(OpenTagStage::Attributes(XmlAttributeStage::AttributeValue(
            ValueStage::Open(_),
        ))) => {
            return false;
        }
        _ => (),
    }

    true
}

fn to_if_req_single(char_val: &char, state: &mut State) {
    if vec!['\n'].iter().any(|x| x == char_val) {
        state.curr_row_num += 1;
    }

    if state.nodes.len() == 0 {
        if is_white_space(char_val) {
            return;
        }

        if char_val != &'<' {
            unexpected_character_error(char_val, state)
        }
        state.nodes.push(Node::new());
        return;
    }

    if should_not_ignore_white_space(char_val, state) {
        return;
    }

    let node_stage = state.nodes[state.nodes.len() - 1].clone().stage.clone();
    match node_stage.clone() {
        NodeStage::OpenTag(open_tag_options) => match open_tag_options {
            OpenTagStage::Key => open_tag_key_stage_open(char_val, state, false),
            OpenTagStage::Attributes(open_tag_stage_attributes) => {
                open_tag_stage_attributes_decision(char_val, state, open_tag_stage_attributes)
            }
            OpenTagStage::TagValueCData(curr_val) => {
                open_tag_stage_cdata_open(char_val, state, &curr_val)
            }
            OpenTagStage::TagValue(node_val) => open_tag_value_stage(char_val, state, &node_val),
            OpenTagStage::AngelBracket => closed_key_is_angle_bracket(char_val, state),
            OpenTagStage::IsEmptyValue => closed_key_is_empty_value(char_val, state),
        },
        NodeStage::ClosedTag(closed_tag_options) => match closed_tag_options {
            ClosedTagStage::ClosedTagOpening => closed_tag_opening(char_val, state),
            ClosedTagStage::ForwardSlash => closed_tag_value_stage_forward_slash(char_val, state),
            ClosedTagStage::Key(init_end_keys) => closed_tag_key_stage(
                char_val,
                state,
                &init_end_keys.open_tag_key,
                &init_end_keys.closed_tag_key,
            ),
            ClosedTagStage::AngelBracket => closed_tag_angle_bracket(char_val, state),
            ClosedTagStage::SibingOrClosing => closed_tag_sibling_or_closing(char_val, state),
        },
    }
}

pub fn xml_to_json(xml_str: &String, to_camel_case: bool) -> Result<String, String> {
    let mut state = State::new();
    state.to_camel_case = to_camel_case;
    for (_, char_val) in xml_str.chars().enumerate() {
        to_if_req_single(&char_val, &mut state);
    }

    if let Some(res_json) = state.str_json {
        Result::Ok(
            res_json
                .chars()
                .skip_while(|x| x != &':')
                .skip(1)
                .collect::<String>()
                .trim()
                .to_string(),
        )
    } else {
        Result::Err("Not enough data to construct a json".to_string())
    }
}
