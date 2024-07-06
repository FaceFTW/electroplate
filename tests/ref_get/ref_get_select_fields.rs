use electroplate::Electroplate;

#[derive(Electroplate())]
struct RefGetStruct {
    i32_field: i32,
    f32_field: f32,
	#[RefGet]
    string_field: String,
    #[RefGet]
    array_field: [i32; 5],
    #[Plating(RefGet, /*Other attributes? */)]
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

    //The following should NOT work
    // assert_eq!(s.i32_field_ref(), &5);
    // assert_eq!(s.f32_field_ref(), &3.0);

    //The following should NOT work
    assert_eq!(s.string_field_ref(), "test");
    assert_eq!(s.array_field_ref(), &[0, 1, 2, 3, 4]);
    assert_eq!(s.vec_field_ref(), &["one", "two", "three"]);
}
