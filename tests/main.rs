use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn run_program(input: &str) -> String {
    let pkg = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set");
    let exe =
        env::var("CARGO_BIN_EXE_".to_string() + &pkg).expect("CARGO_BIN_EXE_<crate> is not set");

    let mut child = Command::new(exe)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to run the program");

    child
        .stdin
        .unwrap()
        .write(input.as_bytes())
        .expect("failed to write to stdin");
    child.stdin = None;

    let output = child
        .wait_with_output()
        .expect("failed to read program output");
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

#[test]
fn example_one_from_spec() {
    assert_eq!(run_program("2 2\n"), "4");
}

#[test]
fn example_two_from_spec() {
    assert_eq!(run_program("1200 2\n"), "1202");
}

#[test]
fn two_positive_small_numbers() {
    assert_eq!(run_program("1 1\n"), "2");
}

#[test]
fn two_zeros() {
    assert_eq!(run_program("0 0\n"), "0");
}

#[test]
fn zero_and_positive() {
    assert_eq!(run_program("0 5\n"), "5");
}

#[test]
fn positive_and_zero() {
    assert_eq!(run_program("7 0\n"), "7");
}

#[test]
fn negative_and_positive() {
    assert_eq!(run_program("-5 3\n"), "-2");
}

#[test]
fn two_negative_numbers() {
    assert_eq!(run_program("-5 -3\n"), "-8");
}

#[test]
fn negative_and_larger_positive() {
    assert_eq!(run_program("-100 200\n"), "100");
}

#[test]
fn positive_and_larger_negative() {
    assert_eq!(run_program("100 -150\n"), "-50");
}

#[test]
fn sum_of_opposite_numbers_is_zero() {
    assert_eq!(run_program("10 -10\n"), "0");
}

#[test]
fn positive_and_negative_positive_result() {
    assert_eq!(run_program("10 -3\n"), "7");
}

#[test]
fn positive_and_negative_negative_result() {
    assert_eq!(run_program("3 -10\n"), "-7");
}

#[test]
fn equal_numbers() {
    assert_eq!(run_program("42 42\n"), "84");
}

#[test]
fn large_positive_numbers() {
    assert_eq!(run_program("123456789 987654321\n"), "1111111110");
}

#[test]
fn seven_digit_numbers() {
    assert_eq!(run_program("1000000 2000000\n"), "3000000");
}

#[test]
fn exceeds_i32_range() {
    assert_eq!(run_program("2147483647 1\n"), "2147483648");
}

#[test]
fn i64_max_boundary() {
    assert_eq!(
        run_program("9223372036854775806 1\n"),
        "9223372036854775807"
    );
}

#[test]
fn i64_min_boundary() {
    assert_eq!(
        run_program("-9223372036854775807 -1\n"),
        "-9223372036854775808"
    );
}

#[test]
fn leading_spaces() {
    assert_eq!(run_program("  3 4\n"), "7");
}

#[test]
fn trailing_spaces() {
    assert_eq!(run_program("3 4  \n"), "7");
}

#[test]
fn multiple_spaces_between_numbers() {
    assert_eq!(run_program("3    4\n"), "7");
}

#[test]
fn tab_separator() {
    assert_eq!(run_program("3\t4\n"), "7");
}

#[test]
fn newline_separator() {
    assert_eq!(run_program("3\n4\n"), "7");
}

#[test]
fn leading_newline() {
    assert_eq!(run_program("\n3 4\n"), "7");
}

#[test]
fn trailing_newline() {
    assert_eq!(run_program("3 4\n\n"), "7");
}
