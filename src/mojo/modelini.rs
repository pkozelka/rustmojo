use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::path::Path;

pub struct ModelIni {
    _sections: HashMap<String, Vec<String>>,
    pub s_info: HashMap<String, String>,
    pub info: ParsedGenericInfo,
    pub columns: Vec<String>,
    pub domains: HashMap<String, DomainInfo>,
}

impl ModelIni {
    pub fn parse<P: AsRef<Path>>(model_ini_path: P) -> Result<ModelIni> {
//        println!("parseFile: {}", (*model_ini_path.as_ref()).to_str());
        let file = File::open(model_ini_path.as_ref().join("model.ini"))?;
        let buf = BufReader::new(file);
        let mut lines= buf.lines();
        let mut sections = ModelIni::read_sections(&mut lines)?;
        let s_info = match &sections.get("[info]") {
            Some(lines) => ModelIni::read_map(lines),
            None => Err(Error::new(ErrorKind::InvalidData, "Section [info] was not found"))
        }?;
        let columns = sections.remove("[columns]").ok_or(Error::new(ErrorKind::InvalidData, "Section [columns] was not found"))?;
        let s_domains = sections.remove("[domains]").ok_or(Error::new(ErrorKind::InvalidData, "Section [domains] was not found"))?;
        let mut domains = HashMap::new();
        for line in s_domains {
            let mut line_parts = line.split(' ');
            //
            let col_index_part = line_parts.next().unwrap();
            let col_index = col_index_part[..col_index_part.len()-1].trim()
                .parse::<usize>()
                .or(Err(Error::new(ErrorKind::InvalidData, "domaininfo index is not int")))?;

            let col_name = columns.get(col_index).ok_or(Error::new(ErrorKind::InvalidData, "domaininfo index is not invalid"))?.to_string();

            let count_part = line_parts.next().unwrap();
            let count = count_part.parse::<usize>().or(Err(Error::new(ErrorKind::InvalidData, "domaininfo size is not int")))?;

            let file_name = String::from(format!("domains/{}", line_parts.next().unwrap()));

            let domain_file = File::open(model_ini_path.as_ref().join(&file_name))?;
            let domain_buf = BufReader::new(domain_file);
            let values: Vec<String> = domain_buf.lines()
                .filter_map(Result::ok)
                .collect();

            if values.len() != count {
                return Err(Error::new(ErrorKind::InvalidData, "domaininfo size does not match number of values"));
            }

            domains.insert( col_name.clone(), DomainInfo {
                col_index,
                col_name,
                file_name,
                values,
            });
        }
        Ok(ModelIni{
            _sections: sections,
            info: ParsedGenericInfo::from(&s_info)?,
            s_info,
            columns,
            domains,
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
            let item = item?;
            let line = item.trim();
            if line.starts_with("[") && line.ends_with("]") {
                // change current section
                section = String::from(line);
                println!("--- section <{}> ---", &section);
                sections.insert(section.clone(), Vec::new());
            } else if line.len() > 0 {
                // add line to the section
                if let Some(lines) = sections.get_mut(&section) {
                    println!(": line : {}", &line);
                    lines.push(String::from(line));
                }
            }
        }
        Ok(sections)
    }
}

fn map_get_boolean(map: &HashMap<String, String>, key: &str) -> Result<bool> {
    match map.get(key) {
        None => Err(Error::new(ErrorKind::InvalidData, format!("Missing entry '{}' in model.ini", key))),
        Some(value) => match value.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Error::new(ErrorKind::InvalidData, format!("Invalid boolean value of '{}' = '{}' in model.ini", key, value))),
        },
    }
}

fn map_get_string(map: &HashMap<String, String>, key: &str) -> Result<String> {
    match map.get(key) {
        None => Err(Error::new(ErrorKind::InvalidData, format!("Missing entry '{}' in model.ini", key))),
        Some(value) => Ok(value.clone()),
    }
}

fn map_get_usize(map: &HashMap<String, String>, key: &str) -> Result<usize> {
    match map.get(key) {
        None => Err(Error::new(ErrorKind::InvalidData, format!("Missing entry '{}' in model.ini", key))),
        Some(value) => match value.parse() {
            Ok(number) => {Ok(number)},
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "cannot parse positive integer")),
        },
    }
}

fn map_get_big_integer(map: &HashMap<String, String>, key: &str) -> Result<i128> {
    match map.get(key) {
        None => Err(Error::new(ErrorKind::InvalidData, format!("Missing entry '{}' in model.ini", key))),
        Some(value) => match value.parse() {
            Ok(number) => {Ok(number)},
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "cannot parse integer")),
        },
    }
}

fn map_get_float(map: &HashMap<String, String>, key: &str) -> Result<f64> {
    match map.get(key) {
        None => Err(Error::new(ErrorKind::InvalidData, format!("Missing entry '{}' in model.ini", key))),
        Some(value) => match value.parse() {
            Ok(number) => {Ok(number)},
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "cannot parse float value")),
        },
    }
}

fn map_get_floats(map: &HashMap<String, String>, key: &str) -> Result<Vec<f64>> {
    match map.get(key) {
        None => Err(Error::new(ErrorKind::InvalidData, format!("Missing entry '{}' in model.ini", key))),
        Some(s) => {
            if s.starts_with("[") && s.ends_with("]") {
                let values = s[1..s.len()-1].split(",");
                let mut rv = Vec::new();
                for value in values {
                    match value.trim().parse() {
                        Ok(number) => rv.push(number),
                        Err(_) => return Err(Error::new(ErrorKind::InvalidData, format!("cannot parse float value: '{}'", value))),
                    }
                }
                Ok(rv)
            } else {
                Err(Error::new(ErrorKind::InvalidData, "array of numbers must be enclosed in '[]'"))
            }

        },
    }
}

pub struct ParsedGenericInfo {
    pub h2o_version: String,
    pub mojo_version: i16,
    pub algo: String,
    pub algorithm: String,
    pub endianness: String,
    pub category: String,
    pub uuid: i128,
    pub supervised: bool,
    pub n_features: usize,
    pub n_classes: usize,
    pub n_columns: usize,
    pub n_domains: usize,
    pub balance_classes: bool,
    pub default_threshold: f64,
    pub prior_class_distrib: Vec<f64>,
    pub model_class_distrib: Vec<f64>,
    pub n_trees: usize,
    pub n_trees_per_class: usize,
    pub distribution: String,
    pub init_f: f64,
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
            h2o_version: map_get_string(map, "h2o_version")?,
            mojo_version: version,
            algo: map_get_string(map, "algo")?,
            algorithm: map_get_string(map, "algorithm")?,
            endianness: map_get_string(map, "endianness")?,
            category: map_get_string(map, "category")?,
            uuid: map_get_big_integer(map, "uuid")?,
            supervised: map_get_boolean(map, "supervised")?,
            n_features: map_get_usize(map, "n_features")?,
            n_classes: map_get_usize(map, "n_classes")?,
            n_columns: map_get_usize(map, "n_columns")?,
            n_domains: map_get_usize(map, "n_domains")?,
            balance_classes: map_get_boolean(map, "balance_classes")?,
            default_threshold: map_get_float(map, "default_threshold")?,
            prior_class_distrib: map_get_floats(map, "prior_class_distrib")?,
            model_class_distrib: map_get_floats(map, "model_class_distrib")?,
            n_trees: map_get_usize(map, "n_trees")?,
            n_trees_per_class: map_get_usize(map, "n_trees_per_class")?,
            distribution: map_get_string(map, "distribution")?,
            init_f: map_get_float(map, "init_f")?,
        })

    }
}

pub struct DomainInfo {
    pub col_index: usize,
    pub col_name: String,
    pub file_name: String,
    pub values: Vec<String>,
}

impl DomainInfo {
}