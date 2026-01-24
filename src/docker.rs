use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::time::Instant;

use crate::models::{Container, ContainerStats};

const CONTAINER_LIST_REQUEST: &str = "GET /containers/json HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
const CONTAINER_STATS_REQUEST: &str = "GET /containers/{}/stats?stream=false&one-shot=true HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";

pub fn parse_response_body(sock: UnixStream) -> String {
    let start = Instant::now();
    let mut reader = BufReader::new(sock);

    let mut content_length = 0;
    let mut is_chunked = false;

    // Read headers
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            break;
        }

        if line.to_lowercase().starts_with("content-length:") {
            content_length =
                line.split(": ").nth(1).unwrap().trim().parse().unwrap();
        }
        if line
            .to_lowercase()
            .starts_with("transfer-encoding: chunked")
        {
            is_chunked = true;
        }
    }

    // Read body
    if is_chunked {
        let mut body = Vec::with_capacity(8192); // Pre-allocate reasonable size
        loop {
            let mut size_line = String::new();
            reader.read_line(&mut size_line).unwrap();
            let chunk_size: usize =
                usize::from_str_radix(size_line.trim(), 16).unwrap();

            if chunk_size == 0 {
                break;
            }

            let mut chunk = vec![0; chunk_size];
            reader.read_exact(&mut chunk).unwrap();
            body.extend_from_slice(&chunk); // More efficient than append
            reader.read_line(&mut String::new()).unwrap(); // trailing \r\n
        }
        println!("Finished reading chunked: {:?}", start.elapsed());
        String::from_utf8(body).unwrap()
    } else {
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        String::from_utf8(body).unwrap()
    }
}

pub fn fetch_container_stats(socket_path: &String) -> Vec<ContainerStats> {
    let start = Instant::now();

    let mut sock = match UnixStream::connect(socket_path) {
        Ok(socket) => socket,
        Err(_) => {
            println!("Ooops!");
            panic!()
        }
    };

    sock.write_all(CONTAINER_LIST_REQUEST.as_bytes()).unwrap();

    let json = parse_response_body(sock);
    let containers: Vec<Container> =
        serde_json::from_str(json.as_str()).unwrap();

    println!("Found {} containers", containers.len());

    let mut container_stats: Vec<ContainerStats> =
        Vec::with_capacity(containers.len());

    for container in containers {
        let mut sock = match UnixStream::connect(socket_path) {
            Ok(socket) => socket,
            Err(_) => {
                println!("Ooops!");
                panic!()
            }
        };
        let message = CONTAINER_STATS_REQUEST.replace("{}", &container.id);
        sock.write_all(message.as_bytes()).unwrap();

        let resp_body = parse_response_body(sock);
        let stats: ContainerStats =
            serde_json::from_str(resp_body.as_str()).unwrap();
        container_stats.push(stats);
    }

    println!("Total execution time: {:?}", start.elapsed());
    container_stats
}
