use std::{
    io::{Read, Write},
    net::{TcpListener, UdpSocket},
    thread::spawn,
};

use interprocess::local_socket::LocalSocketStream;

use crate::{LOCAL_SOCKET_NAME_FROM, LOCAL_SOCKET_NAME_TO};

pub fn tcp_proxy(port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();

    listener.incoming().for_each(|stream| {
        let mut stream = stream.unwrap();
        let mut stream_clone = stream.try_clone().unwrap();

        spawn(move || {
            let mut local_socket = LocalSocketStream::connect(LOCAL_SOCKET_NAME_TO).unwrap();
            let mut buffer = [0; 1024];

            loop {
                local_socket.read(&mut buffer).unwrap();
                stream.write(&buffer).unwrap();
            }
        });

        let mut local_socket = LocalSocketStream::connect(LOCAL_SOCKET_NAME_FROM).unwrap();

        spawn(move || {
            let mut buffer = [0; 1024];

            loop {
                stream_clone.read(&mut buffer).unwrap();
                local_socket.write(&buffer).unwrap();
            }
        });
    });
}

pub fn udp_proxy(port: u16) {
    let udp_socket = UdpSocket::bind(format!("127.0.0.1:{}", port)).unwrap();
    let udp_socket_2 = udp_socket.try_clone().unwrap();

    let mut buffer = [0; 1024];

    let (_, socket_addr) = udp_socket.recv_from(&mut buffer).unwrap();

    spawn(move || {
        let mut local_socket = LocalSocketStream::connect(LOCAL_SOCKET_NAME_TO).unwrap();

        let mut buffer = [0; 1024];

        loop {
            local_socket.read(&mut buffer).unwrap();
            udp_socket.send_to(&buffer, socket_addr).unwrap();
        }
    });

    spawn(move || {
        let mut local_socket = LocalSocketStream::connect(LOCAL_SOCKET_NAME_FROM).unwrap();
        

        loop {
            let (size, _) = udp_socket_2.recv_from(&mut buffer).unwrap();

            local_socket.write(&buffer[..size]).unwrap();
        }
    });

    loop {}
}
