use std::collections::HashMap;
use std::io;
use std::io::Error;
use std::path::Path;

mod modelini;

pub struct Mojo {

}

pub struct BinomialPrediction {
    pub label_index: u32,
    pub label: String,
    pub p0: f64,
    pub p1: f64,
}

impl Mojo {
    pub fn load<P: AsRef<Path>>(p: P) -> io::Result<Mojo> {
        if p.as_ref().is_file() {
            return Err(Error::new(io::ErrorKind::InvalidInput, "Reading zipped mojos is not yet implemented"));
        }
        println!("HELLO FROM Mojo::load('{}')", p.as_ref().to_path_buf().into_os_string().to_str().unwrap());
        let model_ini_path = p.as_ref().join("model.ini");
        println!("modelini: '{}'", model_ini_path.clone().into_os_string().to_str().unwrap());
        let modelini = modelini::ModelIni::parse(model_ini_path)?;

        for (key,value) in modelini.s_info {
            println!("info['{}']='{}'", key, value);
        }

        println!("mojo_version: {}", modelini.info.mojo_version);
        Ok(Mojo{})
    }

    pub fn predict_binomial(&self, _row: HashMap<&str, &str>) -> io::Result<BinomialPrediction>{
//        Err(Error::new(io::ErrorKind::WriteZero, "not implemented"))
        Ok(BinomialPrediction{
            label_index: 0,
            label: String::from("DUMMY"),
            p0: 0.5,
            p1: 0.5,
        })
    }
}
