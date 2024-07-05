use electroplate::Electroplate;

#[derive(Electroplate)]
union DummyUnion {
    f1: u32,
    f2: u32,
}
