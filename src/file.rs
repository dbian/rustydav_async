use quick_xml::name::QName;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
    pub file_type: String,
    pub create_date: String,
    pub modified_date: String,
}

pub fn parse_xml(src: &str) -> Result<Vec<FileInfo>, String> {
    let mut files = Vec::new();
    let mut reader = quick_xml::Reader::from_str(src);
    reader.trim_text(true);
    loop {
        match reader.read_event() {
            Ok(quick_xml::events::Event::Start(ref e)) if e.name() == QName(b"D:response") => {
                let mut file = FileInfo {
                    path: String::new(),
                    name: String::new(),
                    size: 0,
                    is_dir: true,
                    file_type: String::new(),
                    create_date: String::new(),
                    modified_date: String::new(),
                };
                loop {
                    match reader.read_event() {
                        Ok(quick_xml::events::Event::Start(ref e)) => {
                            if e.name() == QName(b"D:href") {
                                file.path = reader
                                    .read_text(e.name())
                                    .map_err(|x| x.to_string())?
                                    .to_string();
                                file.name = file.path.split('/').last().unwrap_or("").to_string();
                                file.file_type =
                                    file.name.split('.').last().unwrap_or("").to_string();
                            } else if e.name() == QName(b"lp1:getcontentlength") {
                                file.size = reader
                                    .read_text(e.name())
                                    .map_err(|x| x.to_string())?
                                    .to_string()
                                    .parse::<u64>()
                                    .map_err(|_| "Failed to parse size")?;
                                file.is_dir = false;
                            } else if e.name() == QName(b"lp1:creationdate") {
                                file.create_date = reader
                                    .read_text(e.name())
                                    .map_err(|x| x.to_string())?
                                    .to_string();
                            } else if e.name() == QName(b"lp1:getlastmodified") {
                                file.modified_date = reader
                                    .read_text(e.name())
                                    .map_err(|x| x.to_string())?
                                    .to_string();
                            }
                        }
                        Ok(quick_xml::events::Event::End(ref e)) => {
                            if e.name() == QName(b"D:response") {
                                files.push(file);
                                break;
                            }
                        }
                        Ok(quick_xml::events::Event::Eof) => return Ok(files),
                        Err(e) => return Err(e.to_string()),
                        _ => (),
                    }
                }
            }
            Ok(quick_xml::events::Event::Start(ref e)) => {}
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => return Err(e.to_string()),
            _ => (),
        }
    }

    Ok(files)
}
