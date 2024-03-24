use self::{open_tag::open_tag_key_stage_open, xml_attribute::xml_attribute_value_closed};

mod cdata;
mod close_tag;
mod open_tag;
mod xml_attribute;

#[derive(Clone, Debug)]
struct Node {
    node_type: NodeType,
    node_key: Option<String>,
    xml_attributes: Vec<XmlAttribute>,
    stage: NodeStage,
}

impl Node {
    fn new() -> Self {
        Self {
            node_type: NodeType::Object(String::new()),
            stage: NodeStage::OpenTag(OpenTagStage::Init),
            xml_attributes: Vec::new(),
            node_key: None,
        }
    }
}

#[derive(Debug, Clone)]
struct XmlAttribute {
    attribute_key: String,
    attribute_val: String,
}

#[derive(Debug, Clone)]
enum NodeType {
    Array(Vec<String>),
    Object(String),
    Number(i32),
    String(String),
}

#[derive(Debug, Clone)]
enum ValueStage {
    Open(String),
    Closed,
}

#[derive(Debug, Clone)]
enum KeyStage {
    Open,
    Closed,
}

#[derive(Debug, Clone)]
enum XmlAttributeStage {
    AttributeKey(ValueStage),
    AttributeValue(ValueStage),
}

#[derive(Debug, Clone)]
enum OpenTagStage {
    Init,
    Key,
    Attributes(XmlAttributeStage),
    TagValueCData(String),
    TagValue(String),
    AngelBracket,
    IsEmptyValue,
}

#[derive(Debug, Clone)]
enum ClosedTagStage {
    ForwardSlash,
    Key,
    AngelBracket,
    SibingOrClosing,
}

#[derive(Debug, Clone)]
enum NodeStage {
    OpenTag(OpenTagStage),
    ClosedTag(ClosedTagStage),
}

#[derive(Debug, Clone)]
struct ArrayNodes {
    node_str_values: Vec<String>,
    key: String,
}

#[derive(Debug)]
struct State {
    nodes: Vec<Node>,
    node_type: Option<NodeType>,
    curr_json: String,
    curr_row_num: i32,
    curr_indent: i32,
}

impl State {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            node_type: None,
            curr_json: String::new(),
            curr_row_num: 1,
            curr_indent: 0,
        }
    }

    fn update_node_stage(&mut self, node_stage: NodeStage) {
        let len = self.nodes.len() - 1;

        self.nodes[len].stage = node_stage;
    }

    fn update_node_type(&mut self, node_type: NodeType) {
        let len = self.nodes.len() - 1;

        self.nodes[len].node_type = Some(node_type);
    }

    fn update_attribute_key(&mut self, key: &String) {
        let len = self.nodes.len() - 1;

        if self.nodes[len].xml_attributes.len() == 0 {
            self.nodes[len].xml_attributes.push(XmlAttribute {
                attribute_key: key.clone(),
                attribute_val: String::new(),
            })
        } else {
            let len_xml_attr = self.nodes[len].xml_attributes.len() - 1;

            self.nodes[len].xml_attributes[len_xml_attr].attribute_key = key.clone();
        }
    }

    fn update_attribute_val(&mut self, str_val: &String) {
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

    fn get_indentation_str(&mut self) -> String {
        let mut tabs_as_str = String::new();
        for _ in 0..self.curr_indent {
            tabs_as_str.push(' ');
        }
        tabs_as_str.pop();

        format!("\n{}", tabs_as_str)
    }
}

fn unexpected_character_error(char_val: &char, state: &State) {
    print!("{:#?}", state);
    panic!(
        "Unexpected character '{}' at row {}",
        char_val, state.curr_row_num
    )
}

fn e_det_tomt_varde_for_i_helvete_javla_fittsugarkuk(char_val: &char) -> bool {
    vec![' ', '\t', '\r'].iter().any(|x| x == char_val)
}

fn should_not_ignore_white_space(char_val: &char, state: &mut State) -> bool {
    if !e_det_tomt_varde_for_i_helvete_javla_fittsugarkuk(char_val) {
        return false;
    }
    match state.nodes[state.nodes.len() - 1].stage.clone() {
        NodeStage::OpenTag(OpenTagStage::Key(KeyStage::Open)) => {
            if let None = state.nodes[state.nodes.len() - 1].node_key {
                return false;
            }
            open_tag_key_stage_open(char_val, state, true);
            true
        }
        NodeStage::OpenTag(OpenTagStage::Attributes(XmlAttributeStage::AttributeValue(
            ValueStage::Closed,
        ))) => {
            xml_attribute_value_closed(char_val, state, true);
            true
        }
        _ => false,
    }
}

fn to_if_req_single(char_val: &char, state: &mut State) {
    if vec!['\n'].iter().any(|x| x == char_val) {
        state.curr_row_num += 1;
        return;
    }

    if state.nodes.len() == 0 {
        if e_det_tomt_varde_for_i_helvete_javla_fittsugarkuk(char_val) {
            return;
        }

        if char_val == &'<' {
            state.nodes.push(Node::new());
        } else {
            unexpected_character_error(char_val, state)
        }

        return;
    }

    if should_not_ignore_white_space(char_val, state) {
        return;
    }

    let node_stage = state.nodes[state.nodes.len() - 1].clone().stage.clone();
    match node_stage {
        NodeStage::OpenTag(_) => todo!(),
        NodeStage::ClosedTag(_) => todo!(),
    }
}

pub fn if_to_json(xml_str: &String) -> Result<String, String> {
    let mut state = State::new();
    for (_, char_val) in xml_str.chars().enumerate() {
        to_if_req_single(&char_val, &mut state);
    }

    Result::Ok(state.curr_json)
}
