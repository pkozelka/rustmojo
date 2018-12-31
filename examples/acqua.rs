extern crate rustmojo as this;

use std::fs::File;
use std::io;
use std::io::Read;

use this::acqua::acquamodel;
use this::mojoreader::MojoInformation;
use this::mojoreader::MojoReader;
use this::mojoreader::ByteArrayReader;

fn read_file(file: &mut File) -> io::Result<acquamodel::Node> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let mut reader = MojoReader::new(MojoInformation::new());
    let mut ba = &mut ByteArrayReader::new(&mut buf.iter());
    reader.read_tree(ba)
}

fn main() {
    let a = read_file(&mut File::open("/home/pk/h2o/h2o-mojo-java/src/test/resources/gbm_v1.00_names.mojo/trees/t00_000.bin").unwrap());
    if let Err(e) = a {
        eprintln!("ERROR: {}", e.to_string())
    }
}
