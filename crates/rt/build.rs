use std::{env::var_os, fs::File, io::Write, path::PathBuf};

fn main() {
    // 링커 스크립트 위치 찾기
    let out = PathBuf::from(var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    let link_x = include_bytes!("link.x");
    let mut f = File::create(out.join("link.x")).unwrap();
    f.write_all(link_x).unwrap();
    println!("cargo:rerun-if-changed=link.x");
}
