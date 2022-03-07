use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;

#[derive(Debug)]
enum Error {
    Connect,
    Write,
    Read,
}

async fn connect() -> Result<TcpStream, Error> {
    println!("Connecting");

    match TcpStream::connect("127.0.0.1:54321").await {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("Err Connect: {:?}", e);
            return Err(Error::Connect);
        }
    }
}

async fn do_it(stream: &mut TcpStream, send_text: String) -> Result<(), Error> {
    println!("Writing");
    match stream.write_all(send_text.as_bytes()).await {
        Ok(s) => s,
        Err(e) => {
            println!("Err Write: {:?}", e);
            return Err(Error::Write);
        }
    };

    println!("Reading");
    let mut buf = vec![0u8; 1024];
    let _ = match stream.read(&mut buf).await {
        Ok(s) => s,
        Err(e) => {
            println!("Err Read: {:?}", e);
            return Err(Error::Read);
        }
    };

    let response = String::from_utf8_lossy(&buf);
    println!("{response}");

    Ok(())
}

fn main() -> Result<(), Error> {
    if let Ok(mut stream) = task::block_on(connect()) {
        loop {
            let comm = input(":> ");
            task::block_on(do_it(&mut stream, comm));
        }
    };

    Ok(())
}

fn input(s: &str) -> String {
    print!("{}", s);
    std::io::Write::flush(&mut std::io::stdout()).expect("stdout().flush()");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim_end().to_string()
}
