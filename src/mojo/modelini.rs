use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;

pub struct ModelIni {
    sections: HashMap<String, Vec<String>>,
    pub s_info: HashMap<String, String>,
    pub info: ParsedGenericInfo,
}

impl ModelIni {
    pub fn parse<P: AsRef<Path>>(model_ini_path: P) -> Result<ModelIni> {
//        println!("parseFile: {}", (*model_ini_path.as_ref()).to_str());
        let file = File::open(model_ini_path)?;
        let buf = BufReader::new(file);
        let mut lines= buf.lines();
        let sections = ModelIni::read_sections(&mut lines)?;
        let s_info = match &sections.get("[info]") {
            Some(info) => ModelIni::read_map(info),
            None => Err(Error::new(ErrorKind::InvalidData, "Section [info] was not found"))
        }?;
        Ok(ModelIni{
            sections: sections,
            info: ParsedGenericInfo::from(&s_info)?,
            s_info: s_info,
        })
    }

    fn read_map(lines: &Vec<String>) -> Result<HashMap<String,String>> {
        let mut map = HashMap::with_capacity(lines.len());
        for line in lines {
            if let Some(n) = line.find("=") {
                let key = String::from(line[0..n].trim());
                let value = String::from(line[n + 1..].trim());
                map.insert(key, value);
            }
            // we ignore lines without colon
        }
        Ok(map)
    }


    ///
    /// Recognize section markers, read lines into a map where each section has a key, and value is sequence of sections.
    ///
    fn read_sections<I>(lines: &mut I) -> Result<HashMap<String, Vec<String>>>
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
        Ok(sections)
    }
}

pub struct ParsedGenericInfo {
    pub mojo_version: i16,
}

impl ParsedGenericInfo {
    fn from(map: &HashMap<String, String>) -> Result<ParsedGenericInfo> {
        let version = match map.get("mojo_version") {
            Some(version) => {
                let numstr: String = version.chars().filter(|c|{c.is_ascii_digit()}).collect();
                let result: i16 = match numstr.parse() {
                    Ok(x) => x,
                    Err(_) => return Err(Error::new(ErrorKind::InvalidData, "cannot parse version number"))
                };
                result
            },
            None => return Err(Error::new(ErrorKind::InvalidData, "Missing key 'version'"))
        };
        Ok(ParsedGenericInfo {
            mojo_version: version,
        })

    }
}
