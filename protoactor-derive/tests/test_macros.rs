#[macro_use]
extern crate protoactor_derive;

#[derive(Message)]
#[rtype(result = "usize")]
struct TestMessage {
    content: String,
}

// clyppy allow dead_code
#[derive(Message)]
struct TestResponse;

#[derive(Message)]
#[rtype(result = "TestResponse")]
pub struct TestRequest2 {
    #[obfuscated]
    pub hidden: String,
    pub visible: String,
    #[hidden]
    invisible: String,
}

#[derive(Message)]
#[rtype(result = "TestResponse")]
pub struct UnNamedStruct(#[obfuscated] String, pub String);

#[derive(Message)]
#[rtype(result = "TestResponse")]
enum TestEnum {
    A,
    B {
        #[obfuscated]
        hidden: u32,
        visible: i128,
    },
    C(u32, i128),
}

#[test]
fn test_message_derive() {
    let test_message = TestMessage {
        content: "42".to_string(),
    };
    assert_eq!(
        "TestMessage { content: \"42\" }".to_string(),
        format!("{:?}", test_message)
    );
}

#[test]
fn test_named_struct_message_derive_with_obfuscated_and_response_type() {
    let test_message = TestRequest2 {
        hidden: "42".to_string(),
        visible: "42".to_string(),
        // below field will not be printed because it is marked as hidden
        invisible: "42".to_string(),
    };
    assert_eq!(
        "TestRequest2 { hidden: \"<obfuscated>\", visible: \"42\" }".to_string(),
        format!("{:?}", test_message)
    );
}

#[test]
fn test_unnamed_struct_message_derive_with_obfuscated_and_response_type() {
    let test_message = UnNamedStruct("42".to_string(), "42".to_string());
    assert_eq!(
        "UnNamedStruct(\"<obfuscated>\", \"42\")".to_string(),
        format!("{:?}", test_message)
    );
}

#[test]
fn test_enum_message_derive_with_obfuscated_and_response_type() {
    let test_message = TestEnum::B {
        hidden: 42,
        visible: 42,
    };
    assert_eq!(
        "TestEnum::B { hidden: \"<obfuscated>\", visible: 42 }".to_string(),
        format!("{:?}", test_message)
    );

    let test_message = TestEnum::C(42, 42);
    assert_eq!(
        "TestEnum::C(42, 42)".to_string(),
        format!("{:?}", test_message)
    );

    let test_message = TestEnum::A;
    assert_eq!("TestEnum::A".to_string(), format!("{:?}", test_message));
}
