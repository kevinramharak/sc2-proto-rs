#[cfg(feature = "protoc-rust")]
use protoc_rust::{Codegen, Customize};
#[cfg(feature = "protoc-rust")]
use std::{env, ffi::OsStr, fs, io::prelude::*, path::Path};

#[cfg(feature = "protoc-rust")]
fn proto_modules(proto_dir: &Path) -> Vec<String> {
    fs::read_dir(proto_dir)
        .expect("Could not read protobuf directory")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() && path.extension() == Some(OsStr::new("proto")) {
                path.file_stem()
                    .and_then(|n| n.to_os_string().into_string().ok())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(feature = "protoc-rust")]
fn main() {
    let in_dir = "./s2client-proto/s2clientprotocol";
    let out_dir = &env::var("OUT_DIR").unwrap();

    // Read list of all input protobuf files
    let input_mods = proto_modules(Path::new(in_dir));
    let input_files: Vec<String> = input_mods
        .iter()
        .map(|s| format!("{}/{}.proto", in_dir, s))
        .collect();

    // Compile protocol buffers
    if let Err(e) = Codegen::new()
        .out_dir(out_dir)
        .include("s2client-proto/")
        .inputs(input_files)
        .customize(Customize {
            expose_fields: Some(true),
            ..Default::default()
        })
        .run()
    {
        panic!("{:#?}", e);
    } else {
        println!("protobufs were generated successfully");
    }

    // Generate the lib.rs source code
    let mut buffer = fs::File::create(format!("{}/{}", out_dir, "lib.rs")).unwrap();
    buffer
        .write_all(
            input_mods
                .iter()
                .map(|s| format!("pub mod {};", s))
                .collect::<Vec<_>>()
                .join("\n")
                .as_bytes(),
        )
        .unwrap();

    // Copy generated *.rs files to "src"
    fs::read_dir(out_dir).unwrap().for_each(|f| {
        let f = f.unwrap();
        fs::copy(f.path(), format!("src/{}", f.file_name().to_str().unwrap())).unwrap();
    });
}

#[cfg(not(feature = "protoc-rust"))]
fn main() {}
