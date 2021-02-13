#[test]
fn command_on_one_line() {
    let code = r#"
def my_echo [arg] { echo $arg }
"#;
    let (result, err) = lex(code, 0);
    println!("{:?}", result);
    assert!(err.is_none());
    let (result, err) = block(result);
    assert!(err.is_none());
    println!("{:?}", result);
}

#[test]
fn command_on_two_lines() {
    let code = r#"
def my_echo [arg] {
echo $arg }
"#;
    let (result, err) = lex(code, 0);
    println!("{:?}", result);
    assert!(err.is_none());
    let (result, err) = block(result);
    assert!(err.is_none());
    println!("{:?}", result);
}

#[test]
fn command_on_three_lines() {
    let code = r#"
def my_echo [arg] {
echo $arg
}
"#;
    let (result, err) = lex(code, 0);
    println!("{:?}", result);
    assert!(err.is_none());
    let (result, err) = block(result);
    assert!(err.is_none());
    println!("{:?}", result);
}

#[test]
fn command_with_parens_broken() {
    let code = r#"
def my_echo [arg]
{ echo $arg }
"#;
    let (result, err) = lex(code, 0);
    println!("{:?}", result);
    assert!(err.is_none());
    let (result, err) = block(result);
    assert!(err.is_none());
    println!("{:?}", result);
}
