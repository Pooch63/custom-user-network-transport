pub use windows::Win32::Networking::WinSock as WinSock;

#[cfg(target_os = "windows")]
pub fn initialize_sockets() {
    use windows::Win32::Networking::WinSock::{ WSADATA, WSAStartup };
    use winsafe::MAKEWORD;
    use windows::Win32::Foundation::NO_ERROR;
    use std::mem::zeroed;

    let mut wsadata: WSADATA = unsafe { zeroed() };
    // Receive startup socket data,() with Winsock version 2.2
    let start_code = unsafe { WSAStartup(MAKEWORD(2, 2), &mut wsadata) };

    if start_code != (NO_ERROR.0 as i32) {
        panic!("ERR (initialize_sockets): WSAStartup failed with error code -> {}", start_code);
    }
}
#[cfg(target_os = "linux")]
pub fn initialize_sockets() {}

#[cfg(target_os = "windows")]
pub fn create_server_socket() -> WinSock::SOCKET {
    use windows::Win32::Networking::WinSock::{
        socket, AF_INET, SOCK_RAW, IPPROTO_ICMP,
        SOCKADDR, SOCKADDR_IN,
        IN_ADDR, INADDR_ANY,
        htons, htonl, bind
    };
    use windows::Win32::Networking::WinSock as winsock;
    
    let m: i32 = AF_INET.0.into();
    let n: i32 = AF_INET.0.to_be().try_into().unwrap();
    println!("{}, {}, {}", AF_INET.0, m, n);

    // With SOCK_RAW to enable raw mode, meaning it can access all network layer packets
    // On Windows, create a raw socket with AF_INET (it will use IPv4)
    let socket = unsafe { socket( AF_INET.0.into(), SOCK_RAW, IPPROTO_ICMP.0.into() )
        .expect("ERR (create_raw_socket) -> Failed to create raw socket") };

    // Bind the socket to the given network interface so it can listen for incoming packets
    // setsockopt is used for this
    // Set the socket options using setsockopt
    // We give it the socket name, as well as the maximum buffer size needed to store the interface
    // name, including the terminating character. This is IFNAMSIZ
    let sockaddr_in = SOCKADDR_IN {
        sin_family: AF_INET,
        // ICMP or IP Raw don't use ports
        sin_port: unsafe { htons(0) },
        sin_addr: IN_ADDR {
            S_un: winsock::IN_ADDR_0 {
                // The server will accept any ocnnection
                S_addr: unsafe { htonl(INADDR_ANY) }
            }
        },
        // Padding so the struct is the same size as sockaddr
        sin_zero: [0i8; 8]
    };
    let interface_bind = unsafe {
        bind(
            socket,
            &sockaddr_in as *const _ as *const SOCKADDR,
            std::mem::size_of::<SOCKADDR_IN>() as i32
        )
    };
    // if interface_bind != 0 {
    //     panic!("Err (create_raw_socket) -> Failed to bind socket to address: {}", std::io::Error::last_os_error());
    // }

    socket
}
#[cfg(target_os = "windows")]
pub fn server_listen(sock: WinSock::SOCKET) {
    use windows::Win32::Networking::WinSock::{
        recvfrom, SOCKADDR, SOCKADDR_IN, SEND_RECV_FLAGS,
        recv
    };

    let mut buffer: [u8; 1024] = [0u8; 1024];
    let mut sender_addr: SOCKADDR_IN = unsafe { std::mem::zeroed() };
    let mut sender_addr_len = std::mem::size_of::<SOCKADDR_IN>() as i32;

    println!("SERVER: Listening on socket {:?}", sock);

    loop {
        // Rust recvfrom only takes 5 arguments (not 6 like the C version),
        // because it doesn't need the buffer length
        // let received = unsafe { recvfrom(
        //     sock,
        //     &mut buffer,
        //     // 0 flags
        //     0,
        //     Some(&mut sender_addr as *mut _ as *mut SOCKADDR),
        //     // Length of the source address is stored in sender_addr_len
        //     Some(&mut sender_addr_len)
        // ) };
        let received = unsafe { recv(
            sock,
            &mut buffer,
            // 0 flags
            SEND_RECV_FLAGS(0),
            // Some(&mut sender_addr as *mut _ as *mut SOCKADDR),
            // // Length of the source address is stored in sender_addr_len
            // Some(&mut sender_addr_len)
        ) };
        if received == -1 {
            eprintln!("ERR(SERVER): Failed to receive data: {}", std::io::Error::last_os_error());
        }
        else if received > 0 {
            println!("SERVER: Received {} bytes", received);
        }
    }
}

pub fn create_client_socket() -> WinSock::SOCKET {
    use windows::Win32::Networking::WinSock::{
        socket, AF_INET, SOCK_RAW, IPPROTO_RAW, IPPROTO_ICMP,
        SOCKADDR, SOCKADDR_IN,
        IN_ADDR, INADDR_ANY,
        htons, htonl, bind
    };
    use windows::Win32::Networking::WinSock as winsock;

    // With SOCK_RAW to enable raw mode, meaning it can access all network layer packets
    // On Windows, create a raw socket with AF_INET (it will use IPv4)
    let socket = unsafe { socket( AF_INET.0.into(), SOCK_RAW, IPPROTO_ICMP.0.into() )
        .expect("ERR (create_raw_socket) -> Failed to create raw socket") };

    // Bind the socket to the given network interface so it can listen for incoming packets
    // setsockopt is used for this
    // Set the socket options using setsockopt
    // We give it the socket name, as well as the maximum buffer size needed to store the interface
    // name, including the terminating character. This is IFNAMSIZ
    // let sockaddr_in = SOCKADDR_IN {
    //     sin_family: AF_INET,
    //     // ICMP or IP Raw don't use ports
    //     sin_port: unsafe { htons(0) },
    //     sin_addr: IN_ADDR {
    //         S_un: winsock::IN_ADDR_0 {
    //             // The server will accept any ocnnection
    //             S_addr: unsafe { htonl(INADDR_ANY) }
    //         }
    //     },
    //     // Padding so the struct is the same size as sockaddr
    //     sin_zero: [0i8; 8]
    // };
    // let interface_bind = unsafe {
    //     bind(
    //         socket,
    //         &sockaddr_in as *const _ as *const SOCKADDR,
    //         std::mem::size_of::<SOCKADDR_IN>() as i32
    //     )
    // };
    // if interface_bind != 0 {
    //     panic!("Err (create_raw_socket) -> Failed to bind socket to address: {}", std::io::Error::last_os_error());
    // }

    socket
}

#[cfg(target_os="windows")]
pub fn queue_server(client: WinSock::SOCKET, server: WinSock::SOCKET) {
    println!("CLIENT: sending data from socket {:?}", client);
    use windows::Win32::Networking::WinSock::{
        SOCKADDR, SOCKADDR_IN, IN_ADDR, IN_ADDR_0,
        htons, sendto, AF_INET
    };
    let sockaddr_in = SOCKADDR_IN {
        sin_family: AF_INET,
        // doesn't matter, ICMP or IP_RAW doesn't use this
        sin_port: unsafe { htons(0) },
        sin_addr: IN_ADDR {
            S_un: IN_ADDR_0 {
                // TConnect to local server
                S_addr: "127.0.0.1".parse::<std::net::Ipv4Addr>().unwrap().into()
            }
        },
        // Padding so the struct is the same size as sockaddr
        sin_zero: [0i8; 8]
    };

    let mut packet: [u8; 16] = [0u8; 16];
    let status = unsafe { 
        sendto(
            // sock,
            client,
            &packet,
            // 0 flags
            0,
            &sockaddr_in as *const _ as *const SOCKADDR,
            std::mem::size_of::<SOCKADDR_IN>() as i32
        )
    };

    if status == -1 {
        println!("Failed to send IP_RAW packet, with error: {}", std::io::Error::last_os_error());
    }
}
#[cfg(target_os = "windows")]
pub fn close_socket(socket: windows::Win32::Networking::WinSock::SOCKET) {
    unsafe { windows::Win32::Networking::WinSock::closesocket(socket) };
}
#[cfg(target_os = "windows")]
pub fn clean_up() {
    unsafe { windows::Win32::Networking::WinSock::WSACleanup() };
}

#[cfg(target_os = "linux")]
pub enum NetworkInterface {
    ETH0
}
#[cfg(target_os = "linux")]
pub struct RawSocket {
    pub interface: NetworkInterface,
}
#[cfg(target_os = "linux")]
fn network_interface_to_string<'a>(interface: NetworkInterface) -> &'a str {
    match interface {
        NetworkInterface::ETH0 => "eth0"
    }
}
#[cfg(target_os = "linux")]
pub fn create_raw_socket(socket_info: RawSocket) {
    use std::ffi::CString;
    use libc::{ socket, AF_PACKET, SOCK_RAW, ETH_P_ALL, setsockopt, SOL_SOCKET, SO_BINDTODEVICE };

    let interface_name = CString::new(
        network_interface_to_string(socket_info.interface)
    ).unwrap();

    // On Linux, create a raw socket with AF_PACKET (it will go all the way down to the packet network layer)
    // And with ETH_P_ALL, enabling it to get all ethernet packets, regardless of the individial protocol
    let socket = unsafe { socket(AF_PACKET, SOCK_RAW, (IP_PROTO as u16).to_be() as i32) };

    let interface_bind = unsafe {
        setsockopt(
            socket,
            SOL_SOCKET,
            SO_BINDTODEVICE,
            interface_name.as_ptr() as *const libc::c_void,
            libc::IFNAMSIZ as libc::socklen_t
        )
    };
}