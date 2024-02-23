use std::error::Error;
use csv;
fn main() {
    std::process::exit(csv_reader());
}
fn csv_reader() -> i32{
    if let Err(e) = read_csv("./sample.csv"){
        eprintln!("{}",e);
    }
    ;
    return 0;
}

fn read_csv(path: &str)-> Result<(),Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;
    for result in reader.records(){
        let record = result?;
        println!("{:?}",record);
    }
    Ok(())
}