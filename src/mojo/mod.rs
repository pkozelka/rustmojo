
use std::path::Path;
use std::io;
use std::collections::HashMap;
use std::io::Error;

pub struct Mojo {

}

pub struct BinomialPrediction {

}

impl Mojo {
    pub fn load<P: AsRef<Path>>(_p: P) -> io::Result<Mojo> {
        println!("HELLO FROM Mojo::load()");
//        Ok(Mojo{})
        Err(Error::new(io::ErrorKind::WriteZero, "not implemented"))
    }

    pub fn predict_binomial(&self, _params: HashMap<String, String>) -> io::Result<bool>{
        Err(Error::new(io::ErrorKind::WriteZero, "not implemented"))
    }
}
