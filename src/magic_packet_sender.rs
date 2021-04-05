use core::panic;
use wake_on_lan_mac_address::MacAddress;

pub struct MagicPacketSender
{}

impl MagicPacketSender
{
    pub fn send_magic_packet(mac_address: &MacAddress) {

        match wake_on_lan::MagicPacket::new(&mac_address.address_array).send() {
            Ok(_) => {},
            Err(e) => panic!("{}", e.to_string()),
        };
        
    }
}