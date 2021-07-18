use libp2p::{NetworkBehaviour, floodsub::{Floodsub, FloodsubEvent}, mdns::{MdnsEvent, TokioMdns}, swarm::NetworkBehaviourEventProcess};
use tokio::sync::mpsc;

pub enum MessageType {
    Invite,
    Transfer,
    Request
}

pub enum EventType {
    Response(Response),
    Input(String)
}


pub struct Request;

pub struct Response {
    data: Message,
    receiver: String
}

pub struct Message {
    mes_type: MessageType,
    data: String
}

#[derive(NetworkBehaviour)]
pub struct MessageBehaviour {
    pub floodsub:Floodsub,
    pub mdns: TokioMdns,
    #[behaviour(ignore)]
    response_sender: mpsc::UnboundedSender<Response>
}


impl NetworkBehaviourEventProcess<FloodsubEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(message) => {
                if message.data[0] == 1 {

                }
                else if message.data[0] == 2 {
                    
                }
            },
            _ => ()
        }
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(discovered_list) => {
                for (peer, _addr) in discovered_list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(expired_list) => {
                for (peer, _addr) in expired_list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}