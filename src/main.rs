pub mod json_to_if;

fn main() {
    println!("This is if_parser!")
}

#[cfg(test)]
mod tests {

    use std::{fs::File, io::Write};

    use crate::json_to_if::to_if_req;

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
}
