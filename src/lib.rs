mod clients;
mod proxies;

const UDP: u8 = 0;
const TCP: u8 = 1;

const LOCAL_SOCKET_NAME_FROM: &str = "from_rmc_plugin:rm_game_share";
const LOCAL_SOCKET_NAME_TO: &str = "to_rmc_plugin:rm_game_share";

#[no_mangle]
pub fn init_server(port: u16, protocol: u8) {
    match protocol {
        UDP => {
            clients::udp_client(port);
        }
        TCP => {
            clients::tcp_client(port);
        }
        _ => {
            println!("Invalid protocol");
        }
    }
}

#[no_mangle]
pub fn init_client(port: u16, protocol: u8) {

    match protocol {
        UDP => {
            proxies::udp_proxy(port);
        }
        TCP => {
            proxies::tcp_proxy(port);
        }
        _ => {
            println!("Invalid protocol");
        }
    }
}
