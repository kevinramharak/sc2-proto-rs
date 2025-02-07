#[cfg(feature = "generate")]
use protobuf_codegen::Codegen;
#[cfg(feature = "generate")]
use protoc_bin_vendored::protoc_bin_path;
#[cfg(feature = "generate")]
use std::{env, ffi::OsStr, fs, path::Path};

#[cfg(feature = "generate")]
fn proto_modules(proto_dir: &Path) -> Vec<String> {
    fs::read_dir(proto_dir)
        .expect("Could not read protobuf directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension() == Some(OsStr::new("proto")) {
                path.file_stem().and_then(|n| n.to_os_string().into_string().ok())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(feature = "generate")]
fn main() -> Result<(), std::io::Error> {
    use protobuf_codegen::Customize;

    let in_dir = "./s2client-proto/s2clientprotocol";
    let out_dir = &env::var("OUT_DIR").unwrap();

    // Read list of all input protobuf files
    let input_mods = proto_modules(Path::new(in_dir));
    let input_files: Vec<String> = input_mods
        .iter()
        .map(|s| format!("{}/{}.proto", in_dir, s))
        .collect();

    // Compile protocol buffers
    Codegen::new()
        .out_dir(out_dir)
        .protoc_path(&protoc_bin_path().unwrap())
        .include("./s2client-proto/")
        .inputs(input_files)
        .customize(
            Customize::default()
                .tokio_bytes(cfg!(feature = "with-bytes"))
                .tokio_bytes_for_string(cfg!(feature = "with-bytes"))
                .lite_runtime(cfg!(feature = "lite-runtime"))
        )
        .run_from_script();

    println!("protobufs were generated successfully");

    // Copy generated *.rs files to "src"
    fs::read_dir(out_dir)?.for_each(|dir_entry| {
        let file = dir_entry.unwrap();
        let file_name = file.file_name();
        fs::copy(file.path(), format!("src/{}", file_name.to_str().unwrap())).unwrap();
    });

    // rename the generated mod.rs to lib.rs
    fs::rename("src/mod.rs", "src/lib.rs")?;

    Ok(())
}

#[cfg(not(feature = "generate"))]
fn main() {
    println!("using pre-generated *.rs files in 'src/'");
}
