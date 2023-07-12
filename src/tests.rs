use crate::*;

#[test]
fn test_args_validation() {
    let args = Args {
        table: None,
        from_file: None,
        logfile_name: Some(String::from("logfile.txt")),
    };
    assert!(std::panic::catch_unwind(|| args.validate()).is_err());

    let args = Args {
        table: Some(String::from("eng-to-ru")),
        from_file: Some(String::from("file.txt")),
        logfile_name: Some(String::from("logfile.txt")),
    };
    assert!(std::panic::catch_unwind(|| args.validate()).is_err());

    let args = Args {
        table: Some(String::from("eng-to-ru")),
        from_file: None,
        logfile_name: Some(String::from("logfile.txt")),
    };
    assert!(std::panic::catch_unwind(|| args.validate()).is_ok());

    let args = Args {
        table: None,
        from_file: Some(String::from("file.txt")),
        logfile_name: Some(String::from("logfile.txt")),
    };
    assert!(std::panic::catch_unwind(|| args.validate()).is_ok());
}

#[test]
fn test_precompiled_translation_tables() {
    let trans_table = TransTable::Precompiled(&ENG_TO_RU);
    assert_eq!(
        trans_table.translate(String::from("some text")),
        "ыщьу еуче",
    );

    let trans_table = TransTable::Precompiled(&ENG_TO_RU);
    assert_eq!(
        trans_table.translate(String::from("not completely энглиш")),
        "тще сщьздуеудн энглиш",
    );

    let trans_table = TransTable::Precompiled(&RU_TO_ENG);
    assert_eq!(
        trans_table.translate(String::from("какой-то текст")),
        "rfrjq-nj ntrcn",
    );

    let trans_table = TransTable::Precompiled(&RU_TO_ENG);
    assert_eq!(
        trans_table.translate(String::from("не всё по-russian")),
        "yt dc` gj-russian",
    );
}

#[test]
fn test_custom_translation_tables() {
    let mut custom_table = HashMap::new();
    custom_table.insert(String::from("a"), String::from("1"));
    custom_table.insert(String::from("b"), String::from("2"));
    let trans_table = TransTable::FromUserInput(custom_table);
    assert_eq!(trans_table.translate(String::from("ab")), "12");
    assert_eq!(trans_table.translate(String::from("abc")), "12c");
    assert_eq!(trans_table.translate(String::from("")), "");

    let mut custom_table = HashMap::new();
    custom_table.insert(String::from("x"), String::from("m"));
    custom_table.insert(String::from("y"), String::from("n"));
    custom_table.insert(String::from("z"), String::from("o"));
    let trans_table = TransTable::FromUserInput(custom_table);
    assert_eq!(trans_table.translate(String::from("xyz")), "mno");
    assert_eq!(trans_table.translate(String::from("xyzw")), "mnow");
    assert_eq!(trans_table.translate(String::from("")), "");
}
