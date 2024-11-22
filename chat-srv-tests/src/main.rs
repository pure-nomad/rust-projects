use std::process::Command;

fn main() {
    for _i in 1..11 {

        let cmd = Command::new("sh")
        .arg("-c")
        .arg("nc localhost 3000")
        .spawn();

    // println!("{:?}",cmd)
    }    
}