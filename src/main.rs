use encoding_rs::UTF_8;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

fn write_metadata_xml(filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let swf_buf = swf::decompress_swf(reader)?;
    let swf = swf::parse_swf(&swf_buf)?;

    for tag in &swf.tags {
        if let swf::Tag::Metadata(i) = tag {
            let metadatapath = format!("{}.metadata.xml", &filename);
            let mut output = File::create(metadatapath)?;
            let metadata = i.to_str_lossy(UTF_8);

            write!(output, "{}", metadata)?;

            return Ok(());
        }
    }

    eprintln!("{} contains no metadata", &filename);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files: Vec<String> = env::args().skip(1).collect();
    for filename in files {
        if let Err(e) = write_metadata_xml(&filename) {
            eprintln!("Error processing file {}: {:?}", filename, e);
        }
    }

    Ok(())
}
