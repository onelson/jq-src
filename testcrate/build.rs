extern crate jq_src;

fn main() {
    let artifacts = jq_src::build().unwrap();
    artifacts.print_cargo_metadata();
}
