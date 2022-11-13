use std::{fs, io};

const PROTO_JS: &'static str = include_str!("web/src/proto.ts");

fn main() -> io::Result<()> {
    let resources_proto = PROTO_JS
        .replace("export const proto: string = `", "")
        .replace("`;", "");

    fs::write("src/resources.proto", resources_proto).expect("failed to write prototype file");

    prost_build::compile_protos(&["src/resources.proto"], &["src/"])?;

    Ok(())
}
