#[derive(Clone, Debug)]
struct Node {
    node_type: Option<NodeType>,
    node_val: Option<String>,
    stage: TagStage,
}

impl Node {
    fn new() -> Self {
        Self {
            node_type: None,
            stage: TagStage::Init,
            node_val: None,
        }
    }
}

#[derive(Debug, Clone)]
enum NodeType {
    Array,
    Object,
    Number,
    String,
}

#[derive(Debug, Clone)]
enum TagStage {
    Init,
    Content,
    End,
}

#[derive(Debug)]
enum NodeStage {
    OpenTag(TagStage),
    ClosedTag(TagStage),
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

fn token_type_json_array_decision(
    token_type_json_array: TokenStage<ArrayValType>,
    char_val: &char,
    state: &mut State,
) {
    match token_type_json_array {
        TokenStage::Opening => json_array_open_case(char_val, state),
        TokenStage::Content(token_stage_content) => {
            token_stage_content_decision(token_stage_content, char_val, state)
        }
        TokenStage::ItemSeparator => json_array_item_separator_case(char_val, state),
        TokenStage::Closing => json_array_closed_case(char_val, state),
    }
}

fn e_det_tomt_varde_for_i_helvete_javla_fittsugarkuk(char_val: &char) -> bool {
    vec![' ', '\t', '\r'].iter().any(|x| x == char_val)
}

fn json_val_open_case_char_empty_val(char_val: &char, state: &mut State) -> bool {
    if !e_det_tomt_varde_for_i_helvete_javla_fittsugarkuk(char_val) {
        return false;
    }
    match state.fields[state.fields.len() - 1].token_type.clone() {
        TokenType::JsonObject(TokenStage::Content(KeyValState::ValState(KeyValType::JsonStr(
            JsonStr::Open(json_str),
        )))) => {
            key_val_json_str_open_case(char_val, state, &json_str);
            true
        }
        TokenType::JsonArray(TokenStage::Content(ArrayValType::JsonStr(JsonStr::Open(
            json_str,
        )))) => {
            array_val_json_str_open_case(char_val, state, &json_str);
            true
        }
        _ => true,
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

        let mut node = Node::new();
        if char_val == &'<' {
        } else {
            unexpected_character_error(char_val, state)
        }

        state.nodes.push(node);
        return;
    }

    if json_val_open_case_char_empty_val(char_val, state) {
        return;
    }

    let node_stage = state.nodes[state.nodes.len() - 1].clone().stage.clone();
    match node_stage {
        TagStage::Init => todo!(),
        TagStage::Content => todo!(),
        TagStage::End => todo!(),
    }
}

pub fn if_to_json(xml_str: &String) -> Result<String, String> {
    let mut state = State::new();
    for (_, char_val) in xml_str.chars().enumerate() {
        to_if_req_single(&char_val, &mut state);
    }

    Result::Ok(state.curr_json)
}
