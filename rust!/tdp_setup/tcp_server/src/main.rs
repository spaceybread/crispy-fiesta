use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
extern crate fuzzy;
extern crate file_system;
use file_system::bucket_loader;
use fuzzy::fuzzyImpl;
use fuzzy::gaussFuzzy;
use fuzzy::bucket::GaussBucket;

fn make_db() {
    let scale = 3;
    let p = 2;
    let dim = 24;
    // Creating the test data 
    let mut all = vec![vec![0.0]; 10000];
    let mut testCases = vec![vec![0.0]; 5];
    let mut tc = 0;

    for i in 0..10000 {
        all[i] = fuzzyImpl::randomVector(dim as usize);

        if i % 2000 == 0 {
            testCases[tc] = all[i].clone();
            tc += 1
        }
    }
    let mut bct = GaussBucket::new(scale, p);
            
    for vec in &all {
        bct.add(vec.clone());
    }
    bucket_loader::make_files_from_bucket(bct.clone());
}



fn parse_vector(input: &str) -> Vec<f64> {
    input
        .split(',')
        .filter_map(|s| s.trim().parse::<f64>().ok())
        .collect()
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; 
    while true {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let rec = String::from_utf8_lossy(&buffer[..bytes_read]);
                if rec.trim().is_empty() {
                    println!("Connection closed!");
                    let response = b"Connection closed!\n";
                    if let Err(e) = stream.write_all(response) {
                        eprintln!("Failed to send response: {}", e);
                    }
                    return;
                }
                let bct = bucket_loader::get_bucket_from_data();
                let recvec = parse_vector(&rec);
                let cands = bucket_loader::handle_queries(bct.clone(), recvec.clone());
                let res = gaussFuzzy::gen(recvec.clone(), bct.scale);
                for can in cands {
                    let rec = gaussFuzzy::recov(res.0.clone(), can.clone(), bct.scale);
                    if rec == res.1 {
                        let mes = format!("{:?}\n", can);
                        if let Err(e) = stream.write_all(mes.as_bytes()) {
                            eprintln!("Failed to send response: {}", e);
                        }
                        println!("{:?}", can);
                    }
                }

                let response = b"Message received\n";
                if let Err(e) = stream.write_all(response) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // connect with nc 127.0.0.1 7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");

                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
