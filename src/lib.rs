use std::{
    ffi::{c_char, CStr, CString},
    str::from_utf8,
};

use regex::Regex;

use crate::json_to_xml::models::{ATTRIBUTES_REGEX_EXPR, IS_ALLOWED_KEY_REGEX_EXPR};

pub mod hebrew_handler;
pub mod json_to_xml;
pub mod xml_to_json;

#[no_mangle]
pub extern "C" fn json_to_xml(
    json_str: *const c_char,
    to_snake_case: bool,
    root_name: *const c_char,
) -> *const c_char {
    IS_ALLOWED_KEY_REGEX_EXPR
        .set(Regex::new(r"^[aA-zZ]").unwrap())
        .unwrap();
    ATTRIBUTES_REGEX_EXPR
        .set(Regex::new(r"_(A|a)(T|t)(T|t)(R|r)(I|i)(B|b)(U|u)(T|t)(E|e)(S|s)$").unwrap())
        .unwrap();

    let root_name_rust_str: String;
    let json_str_rust: String;
    unsafe {
        if json_str.is_null() {
            return CString::new("json string is not valid").unwrap().into_raw();
        }
        if root_name.is_null() {
            return CString::new("root name is not valid").unwrap().into_raw();
        }
        let json_c_str = CStr::from_ptr(json_str);
        json_str_rust = from_utf8(json_c_str.to_bytes()).unwrap().to_string();
        let root_name_c_str = CStr::from_ptr(root_name);
        root_name_rust_str = root_name_c_str.to_str().unwrap().to_string();
    }
    match json_to_xml::json_to_xml(&json_str_rust, to_snake_case, root_name_rust_str) {
        Ok(ok_res) => CString::new(ok_res).unwrap().into_raw(),
        Err(err) => CString::new(err).unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn xml_to_json(xml_str: *const c_char, to_camel_case: bool) -> *const c_char {
    let xml_str_rust: String;
    unsafe {
        if xml_str.is_null() {
            return CString::new("xml string is not valid").unwrap().into_raw();
        }
        let xml_c_str = CStr::from_ptr(xml_str);
        xml_str_rust = from_utf8(xml_c_str.to_bytes()).unwrap().to_string();
    }
    match xml_to_json::xml_to_json(&xml_str_rust, to_camel_case) {
        Ok(ok_res) => CString::new(ok_res).unwrap().into_raw(),
        Err(err) => CString::new(err).unwrap().into_raw(),
    }
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use regex::Regex;

    use crate::{
        json_to_xml::{
            json_to_xml,
            models::{ATTRIBUTES_REGEX_EXPR, IS_ALLOWED_KEY_REGEX_EXPR},
            xml_attributes::xml_attributes_end::remove_str_chunk_by_key,
        },
        xml_to_json::{xml_to_json, Node, NodeStrResult, State, XmlAttribute},
    };

    #[test]
    fn test_parse_to_soap_xml() {
        IS_ALLOWED_KEY_REGEX_EXPR
            .set(Regex::new(r"^[aA-zZ]").unwrap())
            .unwrap();
        ATTRIBUTES_REGEX_EXPR
            .set(Regex::new(r"_(A|a)(T|t)(T|t)(R|r)(I|i)(B|b)(U|u)(T|t)(E|e)(S|s)$").unwrap())
            .unwrap();
        let json = include_str!("./embedded_resources/json_example.json");
        let result = json_to_xml(&json.to_string(), true, "PARAMS_LIST".to_string());

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
        let result = xml_to_json(&json.to_string(), true);

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
        let _ = super::xml_to_json::json_build::get_xml_attributes(
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

        let json_arr = super::xml_to_json::json_build::build_array_json(&nodes, &mut state);
        let json_obj = super::xml_to_json::json_build::build_object_json(&nodes[0], &mut state);
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
        let test_str_expected = include_str!("./embedded_resources/str_removal_test_expected.txt")
            .trim()
            .to_string();
        remove_str_chunk_by_key(
            &mut test_str,
            &"4e315479-9c93-4295-9b06-d604a4bdcc76".to_string(),
        );

        let mut file = File::create("/home/joshua/Public/Tests/comparison_2_xml.xml").unwrap();
        file.write_all(format!("{}\n\n\n{}", test_str.trim(), test_str_expected.trim()).as_bytes())
            .unwrap();

        assert_eq!(test_str.trim(), test_str_expected.trim())
    }
}
