use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        println!("aa{}", input);
        println!("bb{}", 1.to_string());
        // 切掉换行
        let inpts = input.trim();
        if inpts == "q" {
            println!("quit!");
            return Ok(());
        }
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");

        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();

        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into bufer");

        println!(
            "{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
        println!("");
    }
    Ok(())
}
