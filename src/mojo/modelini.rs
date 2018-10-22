use std::io::{Result};
use std::collections::HashMap;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct ModelIni {
    sections: HashMap<String, Vec<String>>
}

impl ModelIni {
    pub fn parse<P: AsRef<Path>>(model_ini_path: P) -> Result<ModelIni> {
//        println!("parseFile: {}", (*model_ini_path.as_ref()).to_str());
        let file = File::open(model_ini_path)?;
        let buf = BufReader::new(file);
        let mut lines= buf.lines();
        ModelIni::append(&mut lines)
    }

    fn append<I>(lines: &mut I) -> Result<ModelIni>
        where I: Iterator<Item = Result<String>>
    {
        let mut sections = HashMap::new();
        let mut section = String::from("");
        for item in lines {
            let line = item?;
            if line.starts_with("[") && line.ends_with("]") {
                // change current section
                section.clone_from(&line);
                println!("--- section <{}> ---", &section);
                sections.insert(section.clone(), Vec::new());
            } else {
                // add line to the section
                if let Some(lines) = sections.get_mut(&section) {
                    let len = lines.len();
                    println!(": line : {}", &line);
                    lines.insert(len, line);
                }
            }
        }
        Ok(ModelIni{
            sections: sections
        })
    }
}
