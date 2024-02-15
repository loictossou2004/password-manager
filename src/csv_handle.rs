use csv::WriterBuilder;
use std::fs::OpenOptions;
use std::io::Result;


pub fn create_csv(file_path: &str) -> Result<()>{
    let mut csv_writer = WriterBuilder::new()
    .from_writer(OpenOptions::new().write(true).append(true).create(true).open(file_path)?);
    if file_metadata(file_path)?.len() == 0 {
        csv_writer.write_record(&["id", "app_name", "username", "password"])?;
    }
    Ok(())
}

pub fn add_in_csv(file_path: &str, id: &usize, app_name: String, username: String, password: String) -> Result<()> {
    let mut csv_writer = WriterBuilder::new()
    .from_writer(OpenOptions::new().write(true).append(true).create(true).open(file_path)?);
    csv_writer.write_record(&[
        id.to_string(),
        app_name.to_string().clone(),
        username.to_string().clone(),
        password.to_string().clone(),
    ])?;
    Ok(())
}

pub fn file_metadata(file_path: &str) -> Result<std::fs::Metadata> {
    Ok(std::fs::metadata(file_path)?)
}