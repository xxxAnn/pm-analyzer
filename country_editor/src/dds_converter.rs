use std::path::Path;
use ddsfile::Dds;
use image_dds::image_from_dds;

pub fn save_to_resources(dds_path: impl Into<String>) -> Result<(), String> {
    let s = dds_path.into();
    let path = Path::new(&s);
    let f = Dds::read(std::fs::File::open(path).map_err(|s| format!("{} Can't read file1", s.to_string()))?).map_err(|_| "Can't read file2".to_owned())?;
    let image = image_from_dds(&f, 0).map_err(|_| "Can't read file3".to_owned())?;
    image.save(format!("resources/{}", path.file_name().ok_or_else(|| "Can't write file".to_owned())?.to_str().ok_or_else(|| "Can't write file")?.replace(".dds", ".png"))).map_err(|t| format!("{}: Can't save file", t.to_string()))?;
    Ok(())
}