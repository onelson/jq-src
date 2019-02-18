extern crate autotools;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("modules")
        .join("jq")
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Build {
    link_static: bool,
    out_dir: Option<PathBuf>,
}

pub struct Artifacts {
    include_dir: PathBuf,
    lib_dir: PathBuf,
    libs: Vec<String>,
    link_static: bool,
}

impl Artifacts {
    pub fn print_link_info(&self) {
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());

        let statik = if self.link_static { "static=" } else { "" };

        for lib in &self.libs {
            println!("cargo:rustc-link-lib={}{}", statik, lib);
        }
    }
}

impl Build {
    pub fn new() -> Build {
        Build {
            link_static: env::var("JQ_NO_STATIC")
                .map(|s| !s.trim().is_empty())
                .unwrap_or(true),
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("jq-build")),
        }
    }

    pub fn link_static(&mut self, value: bool) -> &mut Build {
        self.link_static = value;
        self
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Build {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn build(&mut self) -> Artifacts {
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        let build_dir = out_dir.join("build");
        let inner_dir = build_dir.join("src");

        fs::create_dir_all(&inner_dir).unwrap();
        cp_r(&source_dir(), &inner_dir);

        let mut make_args = vec![];

        if self.link_static {
            make_args.push("LDFLAGS=-all-static".to_string());
        }

        autotools::Config::new(&inner_dir)
            .reconf("-ivf")
            .out_dir(out_dir)
            .disable("-maintainer-mode", None)
            .with("-oniguruma", Some("builtin"))
            .make_args(make_args)
            .build();

        fs::remove_dir_all(&inner_dir).unwrap();

        Artifacts {
            lib_dir: out_dir.join("lib"),
            include_dir: out_dir.join("include"),
            libs: vec!["jq".to_string(), "onig".to_string()],
            link_static: self.link_static,
        }
    }
}

fn cp_r(src: &Path, dst: &Path) {
    for f in fs::read_dir(src).unwrap() {
        let f = f.unwrap();
        let path = f.path();
        let name = path.file_name().unwrap();
        let dst = dst.join(name);
        if f.file_type().unwrap().is_dir() {
            fs::create_dir_all(&dst).unwrap();
            cp_r(&path, &dst);
        } else {
            let _ = fs::remove_file(&dst);
            fs::copy(&path, &dst).unwrap();
        }
    }
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn libs(&self) -> &[String] {
        &self.libs
    }

    pub fn print_cargo_metadata(&self) {
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());
        for lib in self.libs.iter() {
            println!("cargo:rustc-link-lib=static={}", lib);
        }
        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
    }
}
