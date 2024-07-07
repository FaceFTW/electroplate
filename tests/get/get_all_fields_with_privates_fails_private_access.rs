use electroplate::Electroplate;

#[derive(Electroplate(Getters))]
pub struct GetStruct {
    i32_field: i32,
    f32_field: f32,
    #[Private]
    string_field: String,
    array_field: [i32; 5],
    vec_field: Vec<String>,
}

pub fn main() {
    let s = RefGetStruct {
        i32_field: 5,
        f32_field: 3.0,
        string_field: String::from("test"),
        array_field: [0, 1, 2, 3, 4],
        vec_field: vec!["one", "two", "three"],
    };

    //The following should NOT work and should throw a compile error
    assert_eq!(s.string_field(), String::from("test"));
}
