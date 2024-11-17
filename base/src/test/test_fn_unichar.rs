#![allow(clippy::unwrap_used)]

use crate::test::util::new_empty_model;

#[test]
fn simple_cases() {
    let mut model = new_empty_model();
    model._set("A1", "=UNICHAR(\"32\")");
    model._set("A2", "=UNICHAR(\"49\")");
    model._set("A3", "=UNICHAR(49)");
    model._set("A4", "=UNICHAR(84)");
    model._set("A5", "=UNICHAR(1)");
    model._set("A6", "=UNICHAR(12398)");
    model._set("A7", "=UNICHAR(1114109)");
    model._set("A8", "=UNICHAR(TRUE)");

    model.evaluate();

    assert_eq!(model._get_text("A1"), *" ");
    assert_eq!(model._get_text("A2"), *"1");
    assert_eq!(model._get_text("A3"), *"1");
    assert_eq!(model._get_text("A4"), *"T");
    assert_eq!(model._get_text("A5"), *"");
    assert_eq!(model._get_text("A6"), *"の");
    assert_eq!(model._get_text("A7"), *"􏿽");
    assert_eq!(model._get_text("A8"), *"");
}

#[test]
fn test_error_cases() {
    let mut model = new_empty_model();
    model._set("A1", "=UNICHAR(\"\")");
    model._set("A2", "=UNICHAR(#CALC!)");
    model._set("A3", "=UNICHAR(#NAME?)");
    model._set("A4", "=UNICHAR(#VALUE!)");
    model._set("A5", "=UNICHAR(#REF!)");
    model._set("A6", "=UNICHAR(#DIV/0!)");
    model._set("A7", "=UNICHAR(0)");
    model._set("A8", "=UNICHAR(FALSE)");
    model._set("A9", "=UNICHAR(\"T\")");
    model._set("A10", "=UNICHAR(56200)");
    model._set("A11", "=UNICHAR(1114110)");
    model._set("A12", "=UNICHAR(1114111)");
    model._set("A13", "=UNICHAR(1114112)");

    model.evaluate();

    assert_eq!(model._get_text("A1"), *"#VALUE!");
    assert_eq!(model._get_text("A2"), *"#CALC!");
    assert_eq!(model._get_text("A3"), *"#NAME?");
    assert_eq!(model._get_text("A4"), *"#VALUE!");
    assert_eq!(model._get_text("A5"), *"#REF!");
    assert_eq!(model._get_text("A6"), *"#DIV/0!");
    assert_eq!(model._get_text("A7"), *"#VALUE!");
    assert_eq!(model._get_text("A8"), *"#VALUE!");
    assert_eq!(model._get_text("A9"), *"#VALUE!");
    assert_eq!(model._get_text("A10"), *"#N/A");
    assert_eq!(model._get_text("A11"), *"#N/A");
    assert_eq!(model._get_text("A12"), *"#N/A");
    assert_eq!(model._get_text("A13"), *"#VALUE!");
}

#[test]
fn wrong_number_of_arguments() {
    let mut model = new_empty_model();
    model._set("A1", "=UNICHAR()");
    model._set("A2", "=UNICHAR(\"B\",\"A\")");
    model.evaluate();

    assert_eq!(model._get_text("A1"), *"#ERROR!");
    assert_eq!(model._get_text("A2"), *"#ERROR!");
}
