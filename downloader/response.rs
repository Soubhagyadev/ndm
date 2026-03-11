//Response From The Browser
// Logic For Parallel Downloading.
use reqwest::blocking::get;
use std::fs::File;
use std::io::copy;

fn download_file(url: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = get(url)?;

    if !response.status().is_success() {
        return Err(format!("Download failed: {}", response.status()).into());
    }

    let mut file = File::create(path)?;
    copy(&mut response, &mut file)?;

    Ok(())
}

fn main() {
    let url = "https://example.com/file.zip";
    let path = "file.zip";

    match download_file(url, path) {
        Ok(_) => println!("Download completed"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
