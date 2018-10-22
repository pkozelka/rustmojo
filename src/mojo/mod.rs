
use std::path::Path;
use std::io;
use std::collections::HashMap;
use std::io::Error;

mod modelini;

pub struct Mojo {

}

pub struct BinomialPrediction {

}

impl Mojo {
    pub fn load<P: AsRef<Path>>(p: P) -> io::Result<Mojo> {
        if p.as_ref().is_file() {
            return Err(Error::new(io::ErrorKind::InvalidInput, "Reading zipped mojos is not yet implemented"));
        }
        println!("HELLO FROM Mojo::load('{}')", p.as_ref().to_path_buf().into_os_string().to_str().unwrap());
        let model_ini_path = p.as_ref().join("model.ini");
        println!("modelini: '{}'", model_ini_path.clone().into_os_string().to_str().unwrap());
        let _modelini = modelini::ModelIni::parse(model_ini_path)?;

        Ok(Mojo{})
    }

    pub fn predict_binomial(&self, _params: HashMap<String, String>) -> io::Result<bool>{
        Err(Error::new(io::ErrorKind::WriteZero, "not implemented"))
    }
}
