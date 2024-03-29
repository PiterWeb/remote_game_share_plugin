use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpStream, UdpSocket},
    thread::spawn,
};

use interprocess::local_socket::LocalSocketStream;

use crate::{LOCAL_SOCKET_NAME_FROM, LOCAL_SOCKET_NAME_TO};



pub fn tcp_client(port: u16) {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

    let mut tcp_socket = TcpStream::connect(socket_addr).unwrap();
    let mut tcp_socket_2 = tcp_socket.try_clone().unwrap();

    spawn(move || {
        let mut local_socket =
            LocalSocketStream::connect(LOCAL_SOCKET_NAME_TO).unwrap();
        let mut buffer = [0; 1024];

        loop {
            local_socket.read(&mut buffer).unwrap();
            tcp_socket.write(&buffer).unwrap();
        }
    });

    let mut local_socket =
        LocalSocketStream::connect(LOCAL_SOCKET_NAME_FROM).unwrap();

    let mut buffer = [0; 1024];

    loop {
        tcp_socket_2.read(&mut buffer).unwrap();
        local_socket.write(&buffer).unwrap();
    }


}



pub fn udp_client(port: u16) {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

    let udp_socket = UdpSocket::bind(socket_addr).unwrap();
    let udp_socket_2 = udp_socket.try_clone().unwrap();

    spawn(move || {
        let mut local_socket =
            LocalSocketStream::connect(LOCAL_SOCKET_NAME_TO).unwrap();
        let mut buffer = [0; 1024];

        loop {
            local_socket.read(&mut buffer).unwrap();
            udp_socket.send_to(&buffer, socket_addr).unwrap();
        }
    });

    let mut local_socket =
        LocalSocketStream::connect(LOCAL_SOCKET_NAME_FROM).unwrap();
    let mut buffer = [0; 1024];

    loop {
        udp_socket_2.recv_from(&mut buffer).unwrap();

        local_socket.write(&buffer).unwrap();
    }
}
