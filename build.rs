extern crate autotools;

fn main() {
    let jq = autotools::Config::new("modules/jq")
        .reconf("-ivf")
        .with("-oniguruma", Some("builtin"))
        .disable("-maintainer-mode", None)
        .make_args(vec!["LDFLAGS=-all-static".to_string()])
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        jq.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=onig");
    println!("cargo:rustc-link-lib=static=jq");
}
