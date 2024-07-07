use electroplate::Electroplate;

#[derive(Electroplate(RefGetters))]
struct RefGetStruct {
    i32_field: i32,
    f32_field: f32,
    #[Private]
    string_field: String,
    array_field: [i32; 5],
    vec_field: Vec<String>,
}

fn main() {
    let s = RefGetStruct {
        i32_field: 5,
        f32_field: 3.0,
        string_field: String::from("test"),
        array_field: [0, 1, 2, 3, 4],
        vec_field: vec!["one", "two", "three"],
    };

    //The following should NOT work and should compiler error
    assert_eq!(s.string_field_ref(), "test");
}
