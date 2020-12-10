use clap::Clap;
use readability::extractor::extract;
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;

/// HTML to Readability CLI
#[derive(Clap)]
#[clap(version = "1.0", author = "Dustin K <dustin@knopoff.dev")]
struct Args {
    file: PathBuf,
    url: url::Url,
}

fn main() -> Result<(), anyhow::Error> {
    let opts: Args = Args::parse();
    let file = File::open(&opts.file)?;
    let mut reader = BufReader::new(file);
    let product = extract(&mut reader, &opts.url)?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(format!("<h1>{}</h1>", product.title).as_bytes())?;
    handle.write_all(product.content.as_bytes())?;
    handle.write_all(
        format!(r#"<p>Source: <a href="{}">{}</a>"#, opts.url, product.title).as_bytes(),
    )?;
    Ok(())
}
