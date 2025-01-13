use utils_derive::MyMacro;

#[derive(MyMacro)]
struct MyStruct;

#[test]
fn test() {
    assert_eq!(MyStruct::hello(), "Hello from MyStruct");
}
