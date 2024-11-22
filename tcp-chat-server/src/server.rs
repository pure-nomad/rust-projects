pub mod server_handler {
    use std::{io::{BufRead, BufReader, Write,Result}, net::{TcpListener,TcpStream,Shutdown}, thread};
    use std::sync::{Arc, Mutex,atomic::{AtomicUsize,Ordering}};

    const NICKNAME_MSG: &str = "Enter Nickname: ";
    const ENTER_MSG: &str = "\n> ";

    static CLIENT_COUNT: AtomicUsize = AtomicUsize::new(0);

    pub fn start_server() -> Result<()> {

        let conn_vec: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
        // make listener on localhost
        let listener = TcpListener::bind("127.0.0.1:3000")?;

        println!("[+] Server Started");
        // start connection receiver loop
            for conn in listener.incoming() {
                match conn {

                    Ok(mut conn) => {
                        CLIENT_COUNT.fetch_add(1,Ordering::SeqCst);
                        let stream = conn.try_clone()?;

                        conn_vec.lock().unwrap().push(stream);

                        if CLIENT_COUNT.load(Ordering::SeqCst) > 10 {

                            println!("Too many connections! Server overloaded...");
                            for client in conn_vec.lock().unwrap().iter() {
                                broadcast_msg(client,"Server overloaded".to_string())?;
                            }
                            std::process::exit(0);
                        }

                        println!("Current client count: {}",CLIENT_COUNT.load(Ordering::SeqCst));

                        let conn_vec_clone = Arc::clone(&conn_vec);
                        thread::spawn(move||{
                            let welcome_msg = format!("[-] Welcome! {}/10\n",CLIENT_COUNT.load(Ordering::SeqCst));
                            let _ = conn.write_all(welcome_msg.as_bytes());
                            let _ = conn.flush();
                            let _ = handle_msg(&conn,conn_vec_clone);
                        });



                    },

                    Err(e) => println!("Connection failed: {e:?}"),
                }



            }

        Ok(())
    }

    fn broadcast_msg(mut connection: &TcpStream, message: String) -> Result<()> {

        let _ = connection.write_all(message.as_bytes());
        let _ = connection.flush();

        Ok(())
    }

    fn handle_msg(mut conn: &TcpStream,client_vec: Arc<Mutex<Vec<TcpStream>>>) -> Result<()> {

        let mut uname_inp = BufReader::new(conn);

        conn.write_all(NICKNAME_MSG.as_bytes())?;

        conn.flush()?;

        let uname: Vec<u8> = uname_inp.fill_buf()?.to_vec();

        let uname_str = String::from_utf8_lossy(&uname);

        println!("User {:?} has joined!\n",uname_str);

        for client in client_vec.lock().unwrap().iter() {
            broadcast_msg(client,format!("\nUser {} has joined!\n", uname_str.trim()))?;
        }

        loop {

            let mut msg_inp = BufReader::new(conn);

            conn.write_all(ENTER_MSG.as_bytes())?;

            conn.flush()?;

            let msg: Vec<u8> = msg_inp.fill_buf()?.to_vec();



            if let Ok(dmsg) = String::from_utf8(msg.clone()) {

                if dmsg == String::from("exit\n") {

                    for client in client_vec.lock().unwrap().iter() {
                        broadcast_msg(client,format!("User: {} left the chat",uname_str.trim()))?;
                    }
                    CLIENT_COUNT.fetch_min(1,Ordering::SeqCst);
                    conn.shutdown(Shutdown::Both)?;

                    break;
                }
            }

            let msg_string = String::from_utf8_lossy(&msg);
            println!("Message: {:?}\n",msg_string);
            for client in client_vec.lock().unwrap().iter() {
                broadcast_msg(client,format!("\n{}: {}\n",uname_str.trim(), msg_string.trim()))?;
            }
        }

        // we will have to make a broadcast msg function in order to fix our problem bc TCP doesn't support it.
        // we need to have an iteration of our threads , their tcp stream, and write to each thread's tcp stream.



        Ok(())
    }


}