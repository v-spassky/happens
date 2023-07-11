use crate::*;

#[test]
fn test_get_substitute_eng_to_ru() {
    let user_input = "qwerty".to_string();
    let table_name = "eng-to-ru".to_string();
    assert_eq!(get_substitute(user_input, table_name), "йцукен".to_string());
}

#[test]
fn test_get_substitute_ru_to_eng() {
    let user_input = "йцукен".to_string();
    let table_name = "ru-to-eng".to_string();
    assert_eq!(get_substitute(user_input, table_name), "qwerty".to_string());
}

#[test]
fn test_get_substitute_mixed() {
    let user_input = "qweЙЦУ".to_string();
    let table_name = "eng-to-ru".to_string();
    assert_eq!(get_substitute(user_input, table_name), "йцуЙЦУ".to_string());
}

#[test]
#[should_panic(expected = "Unknown table name: unknown")]
fn test_get_substitute_unknown_table() {
    let user_input = "qwerty".to_string();
    let table_name = "unknown".to_string();
    get_substitute(user_input, table_name);
}
