fn main() {
    // rust-embed needs the folder to exist at compile time. The web UI itself is
    // built separately with `npm run build` in `web/`.
    std::fs::create_dir_all("web/dist").expect("could not create web/dist");
    println!("cargo:rerun-if-changed=web/dist");
}
