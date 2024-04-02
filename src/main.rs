use regex::Regex;

use crate::json_to_if::models::{ATTRIBUTES_REGEX_EXPR, IS_ALLOWED_KEY_REGEX_EXPR};

pub mod if_to_json;
pub mod json_to_if;

fn main() {
    println!("This is if_parser!");
    IS_ALLOWED_KEY_REGEX_EXPR
        .set(Regex::new(r"^[aA-zZ]").unwrap())
        .unwrap();
    ATTRIBUTES_REGEX_EXPR
        .set(Regex::new(r"_(A|a)(T|t)(T|t)(R|r)(I|i)(B|b)(U|u)(T|t)(E|e)(S|s)$").unwrap())
        .unwrap();
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use crate::{
        if_to_json::{if_to_json, Node, NodeStrResult, State, XmlAttribute},
        json_to_if::{to_if_req, xml_attributes::xml_attributes_end::remove_str_chunk_by_key},
    };

    #[test]
    fn test_parse_to_soap_xml() {
        let json = include_str!("./embedded_resources/json_example.json");
        let result = to_if_req(&json.to_string());

        match result {
            Ok(ok_res) => {
                let mut file = File::create("/home/joshua/Public/Tests/json_to_if.xml").unwrap();

                file.write_all(ok_res.as_bytes()).unwrap();
                assert!(true)
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_parse_to_json() {
        let json = include_str!("./embedded_resources/xml_example.xml");
        let result = if_to_json(&json.to_string());

        match result {
            Ok(ok_res) => {
                let mut file = File::create("/home/joshua/Public/Tests/if_to_json.json").unwrap();

                file.write_all(ok_res.as_bytes()).unwrap();
                assert!(true)
            }
            Err(err_msg) => assert!(false, "{}", err_msg),
        }
    }

    #[test]
    fn test_json_building() {
        let json_nested = include_str!("./embedded_resources/json_example_nested.json");
        let key = "test".to_string();
        let attributes = vec![
            XmlAttribute {
                attribute_key: "name".to_string(),
                attribute_val: "Joshua".to_string(),
            },
            XmlAttribute {
                attribute_key: "hobby".to_string(),
                attribute_val: "piano".to_string(),
            },
            XmlAttribute {
                attribute_key: "age".to_string(),
                attribute_val: "29".to_string(),
            },
        ];
        let mut state = State::new();
        state.curr_indent = 2;
        let node = Node::new();
        state.nodes.push(node);
        let mut attr_str = String::new();
        let _ = super::if_to_json::json_build::get_xml_attributes(
            &attributes,
            &mut state,
            &mut attr_str,
        );

        let nodes = vec![
            NodeStrResult {
                str_value: json_nested.trim_end().to_string().clone(),
                xml_attributes_str: attr_str.clone(),
                key: key.clone(),
                is_object: true,
            },
            NodeStrResult {
                str_value: json_nested.to_string().trim_end().to_string().clone(),
                xml_attributes_str: "  null".to_string(),
                key: key.clone(),
                is_object: true,
            },
        ];

        let json_arr = super::if_to_json::json_build::build_array_json(&nodes, &mut state);
        let json_obj = super::if_to_json::json_build::build_object_json(&nodes[0], &mut state);
        let mut json_arr_file =
            File::create("/home/joshua/Public/Tests/json_arr_example_res.json").unwrap();
        let mut json_obj_file =
            File::create("/home/joshua/Public/Tests/json_obj_example_res.json").unwrap();

        json_arr_file.write_all(json_arr.as_bytes()).unwrap();
        json_obj_file.write_all(json_obj.as_bytes()).unwrap();
        assert!(true)
    }

    #[test]
    fn test_removal_str_chunk_between_guids() {
        let mut test_str = include_str!("./embedded_resources/str_removal_test.txt")
            .trim()
            .to_string();
        let guid = uuid::Uuid::new_v4().to_string();
        test_str = test_str.replace("[GUID]", guid.as_str());
        remove_str_chunk_by_key(&mut test_str, &guid);
        assert_eq!(test_str, "JoshuaGoldman")
    }
}
