use std::collections::HashMap;

pub fn hebrew_str_to_non_hebrew(str_val: &mut String, to_non_hebrew: bool) {
    let guid_to_hebrew_map = HashMap::from([
        ("810cc936-b443-46d9-ac5e-646e2f6fea68", "ק"),
        ("1b1a3310-8c1a-4823-bb08-55a69a836387", "ר"),
        ("0befe91b-b117-426e-bb0f-f4a6652f4711", "א"),
        ("db5d4f5b-3670-4766-b2c5-a8e187226276", "ט"),
        ("fc70d64d-79d1-4119-932d-66517e8bd1a9", "ו"),
        ("ab42380a-ed4c-4931-af0a-d38be8029298", "ן"),
        ("f8601c36-fc31-4032-a25a-7c4284a66baf", "ם"),
        ("5e2ba150-1369-418a-8faa-995960b41c05", "פ"),
        ("6a20a60a-3be8-42cc-8e8c-9aff51128e45", "ף"),
        ("d1a8c944-a623-45c8-8f57-811d302376d6", "ך"),
        ("3a53210c-9d73-496c-a110-b9caa9837ddd", "ל"),
        ("c4f6348b-f69b-4bb6-9cc5-a33b39b36bfe", "ח"),
        ("7fd481ce-294c-4112-909c-9bf5eaa38351", "י"),
        ("e540ba32-f6b0-43c6-92af-28a4404a2675", "ע"),
        ("a80d5945-81d1-4dbf-9188-7e1af8ac809c", "כ"),
        ("fc96d236-4660-4afd-93c7-39415d39cdfd", "ג"),
        ("2d3cab03-b04a-45b2-ae09-8331eea51094", "ד"),
        ("09d07874-5797-4255-a769-f900b047b3a2", "ש"),
        ("b12df03d-4b36-4003-bb4f-4ec5443ffdcd", "ז"),
        ("a895da10-5605-455a-821d-019b55ec54b3", "ס"),
        ("3badcfc5-8923-4465-9fd3-af5c77111384", "ב"),
        ("d600421a-c5b2-41cf-ab6d-949840320c83", "ה"),
        ("6c08bdc0-33c0-449b-b20b-b6cb8a22e186", "נ"),
        ("f0c89f4a-7f67-49d0-8bc0-0f8fd7e73fbc", "מ"),
        ("0a05976a-14b4-44cf-8cf8-f9c947ec884e", "צ"),
        ("9f9d74af-99cc-4242-b9e9-ce1bb29b193d", "ת"),
        ("ddfe1abc-a747-4f30-9a6c-15a5340bf45a", "ץ"),
    ]);

    if to_non_hebrew {
        for (_, (key, key_val)) in guid_to_hebrew_map.iter().enumerate() {
            *str_val = str_val.replace(key_val, key);
        }
    } else {
        for (_, (key, key_val)) in guid_to_hebrew_map.iter().enumerate() {
            *str_val = str_val.replace(key, key_val);
        }
    }
}
