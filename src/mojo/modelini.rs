use std::io::{Result, ErrorKind};
use std::io::Lines;
use std::collections::HashMap;
use std::io::Error;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

pub struct ModelIni {
    sections: HashMap<String, Vec<String>>
}

impl ModelIni {
    pub fn parseFile<P: AsRef<Path>>(model_ini_path: P) -> Result<ModelIni> {
//        println!("parseFile: {}", (*model_ini_path.as_ref()).to_str());
        let file = File::open(model_ini_path)?;
        let buf = BufReader::new(file);
        let mut lines= buf.lines();
        ModelIni::parse(&mut lines)
    }

    pub fn parse<I>(lines: &mut I) -> Result<ModelIni>
        where I: Iterator<Item = Result<String>>
    {
        let mut sections = HashMap::new();
        let mut section = String::from("");
        for item in lines {
            let line = item?;
            if line.starts_with("[") && line.ends_with("]") {
                // change current section
                &mut section.clone_from(&mut String::from(line));
                println!("--- section <{}> ---", &section);
                sections.insert(section.clone(), vec![]);
            } else {
                // add line to the section
                let mut sectionLines = sections.get_mut(&section);
                if let Some(lines) = sectionLines {
                    let len = lines.len();
                    lines.insert(len, line);
                }
            }
        }
        Ok(ModelIni{
            sections: sections
        })
    }
}
