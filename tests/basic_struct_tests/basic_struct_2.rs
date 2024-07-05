use electroplate::Electroplate;

enum BasicEnum1 {
    Variant0,
    Variant1(i32),
    Variant2 { string_field: String },
}

#[derive(Electroplate)]
struct BasicStruct2 {
    enum_field: BasicEnum1,
}

pub fn main() {
    let instance = BasicStruct2 {
        enum_field: BasicEnum1::Variant0,
    };
    assert_eq!(instance.enum_field(), BasicEnum1::Variant0);

    let instance2 = BasicStruct2 {
        enum_field: BasicEnum1::Variant1(5),
    };

    assert_eq!(instance2.enum_field(), BasicEnum1::Variant1(5));

    let instance3 = BasicStruct2 {
        enum_field: BasicEnum1::Variant2 {
            string_field: String::from("test string"),
        },
    };
    assert_eq!(
        instance2.enum_field(),
        BasicEnum1::Variant2 {
            string_field: String::from("test string")
        }
    )
}
