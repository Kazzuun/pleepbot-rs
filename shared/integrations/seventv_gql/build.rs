fn main() {
    cynic_codegen::register_schema("seventv")
        .from_sdl_file("schemas/seventv.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
