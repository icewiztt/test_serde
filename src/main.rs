use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::fs::File;
use std::io::{BufReader, BufWriter};


#[derive(Debug, Serialize, Deserialize)]
struct Move{
    direction: i32,
    points: Vec<i32>
}

fn main() -> Result<()>{
    let a = Move{
        direction: 1,
        points: Vec::from([1,2,3])
    };
    println!("{:?}", a);

    let file = File::open("foo.txt").unwrap();
    let writer = BufWriter::new(&file);

    serde_json::to_writer(writer, &a)?;

    let reader = BufReader::new(&file);
    let b: Move = serde_json::from_reader(reader)?;

    println!("{:?}", b);
    Ok(())
}
