#[cfg(test)]
use std::error::Error;

#[cfg(test)]
pub async fn load_lines(file_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    use std::path::PathBuf;
    use tokio::{fs::File, io::{BufReader, AsyncBufReadExt}};
    use tokio_stream::{wrappers::LinesStream, StreamExt};

    let mut loaded_lines = Vec::new();

    let file_name: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", file_name].iter().collect();

    let file = File::open(file_name).await?;
    let buffered = BufReader::new(file);

    let lines = LinesStream::new(buffered.lines());
    tokio::pin!(lines);
    while let Some(line) = lines.next().await {
        match line {
            Ok(l) => loaded_lines.push(l),
            Err(e) => return Err(Box::new(e))
        }
    }

    Ok(loaded_lines)
}