extern crate jq_src;

fn main() {
    let artifacts = jq_src::Build::new().build();
    artifacts.print_link_info();
}
