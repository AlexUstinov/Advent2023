use std::error::Error;
use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;


pub struct Solution;

impl Solution {
    pub async fn find_calibration_sum(input_file: PathBuf) -> Result<i32, Box<dyn Error>> {
        const NEEDLES: [&[u8]; 10] = [
            "".as_bytes(),
            "one".as_bytes(),
            "two".as_bytes(),
            "three".as_bytes(),
            "four".as_bytes(),
            "five".as_bytes(),
            "six".as_bytes(),
            "seven".as_bytes(),
            "eight".as_bytes(),
            "nine".as_bytes()];
        let file = File::open(input_file).await?;
        let buffered = BufReader::new(file);

        let lines = LinesStream::new(buffered.lines());
    
        let mut sum = 0;
        tokio::pin!(lines);
        while let Some(line) = lines.next().await {
            match line {
                Ok(l) => {
                    let bytes = l.as_bytes();
                    let mut i = 0;
                    let (mut first, mut last) = (-1, 0);
                    while i < bytes.len() {
                        let c = bytes[i];
                        let d = if c >= b'0' && c <= b'9' {
                            (c - b'0') as i32
                        } else {
                            let mut digit = -1;
                            for (d, needle) in NEEDLES.iter().enumerate().skip(1) {
                                if bytes[i..].starts_with(needle) {
                                    digit = d as i32;
                                    break;
                                }
                            }
                            digit
                        };
                        if d>=0 {
                            last = d;
                            if first < 0 {
                                first = last * 10;
                            }
                        }
                        i += 1;
                    }
                    if first >=0 {
                        sum += first + last;
                    }
                },
                Err(e) => return Err(Box::new(e))
            }
        }
    
        Ok(sum)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn solve() {
        let file_name: PathBuf = [env!("CARGO_MANIFEST_DIR"), "input", "day_01.txt"].iter().collect();

        let sum = Solution::find_calibration_sum(file_name).await;
        println!("{sum:?}");
    }
}