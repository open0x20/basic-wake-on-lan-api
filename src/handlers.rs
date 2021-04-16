use actix_web::{ web, HttpResponse };
use wake_on_lan_mac_address::MacAddress;

#[path = "./request_dto.rs"] mod request_dto;
use request_dto::RequestDto;

#[path ="./magic_packet_sender.rs"] mod magic_packet_sender;
use magic_packet_sender::MagicPacketSender;

pub struct WakeOnLanHandler {}

impl WakeOnLanHandler
{
    pub async fn wake(item: web::Json<RequestDto>) -> HttpResponse
    {
        let mac_address_str: &str = item.0.mac_address.as_str();
        let mac: MacAddress = MacAddress::from_string_slice(&mac_address_str).unwrap();

        MagicPacketSender::send_magic_packet(&mac);

        HttpResponse::Ok().finish()
    }
}