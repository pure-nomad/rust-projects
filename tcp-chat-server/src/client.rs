pub mod client_handler {
    use std::{io::{stdin, Write},net::TcpStream};
    

    pub fn start_client() -> std::io::Result<()> {
        println!("Starting Client..");

        // create stream

        let stream = TcpStream::connect("127.0.0.1:3000");

        match stream {
            Ok(res) => {let _ = handle_stream(res);},
            Err(e) => println!("Error: {:?}",e),
        }

        Ok(())
    }

    fn handle_stream(mut stream: TcpStream) -> std::io::Result<()> {
        let ip = stream.peer_addr()?;
        println!("Connected to server: {}:{}",ip.ip(),ip.port());

        loop {
            println!("Enter message below:");
            let mut msg = String::new();
            stdin().read_line(&mut msg).expect("Please enter a string.");
            if msg.eq_ignore_ascii_case("exit") {
                println!("exiting...");
                break;
            }
            
            let _ = stream.write_all(msg.as_bytes())?;
            stream.flush()?;
            continue;
        }

        // TODO: figure out why we can only see the first message on the server.
        // RESOLUTION: use a loop dumbass


        Ok(())
    }
}