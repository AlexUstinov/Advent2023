use std::error::Error;
use std::path::PathBuf;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;


pub struct Solution;

impl Solution {
    pub async fn find_calibration_sum(input_file: PathBuf) -> Result<i32, Box<dyn Error>> {

        let file = File::open(input_file).await?;
        let buffered = BufReader::new(file);

        let lines = LinesStream::new(buffered.lines());
    
        let mut sum = 0;
        tokio::pin!(lines);
        while let Some(line) = lines.next().await {
            match line {
                Ok(l) => {
                    let (mut first, mut last) = (-1, 0);
                    for c in l.bytes() {
                        match c {
                            d @ b'0'..=b'9' => {
                                last = (d - b'0') as i32;
                                if first < 0 {
                                    first = last * 10;
                                }
                            },
                            _ => continue
                        }
                    }
                    if first >=0 {
                        sum += first + last;
                    }
                },
                Err(e) => return Err(Box::new(e))
            }
        }
    
        Ok(sum)

        // let stream = response.bytes_stream()
        //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e));
        // let reader = tokio_util::io::StreamReader::new(stream);
        // let buffered = BufReader::new(reader);
    
        // let lines = LinesStream::new(buffered.lines());
    
        // let mut sum = 0;
        // tokio::pin!(lines);
        // while let Some(line) = lines.next().await {
        //     match line {
        //         Ok(l) => {
        //             let (mut first, mut last) = (-1, 0);
        //             for c in l.bytes() {
        //                 match c {
        //                     d @ b'0'..=b'9' => {
        //                         last = (d - b'0') as i32;
        //                         if first < 0 {
        //                             first = last * 10;
        //                         }
        //                     },
        //                     _ => continue
        //                 }
        //             }
        //             sum += first + last;
        //         },
        //         Err(e) => return Err(Box::new(e))
        //     }
        // }
    
        // Ok(sum)
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