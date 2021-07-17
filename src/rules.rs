use libp2p::{NetworkBehaviour, floodsub::{Floodsub, FloodsubEvent}, mdns::{MdnsEvent, TokioMdns}, swarm::NetworkBehaviourEventProcess};
use tokio::sync::mpsc;

pub enum MessageType {
    Invite,
    Transfer,
    Request
}

pub struct Message {
    mes_type: MessageType,
    data: String
}

#[derive(NetworkBehaviour)]
pub struct MessageBehaviour {
    floodsub:Floodsub,
    mdns: TokioMdns,
    #[behaviour(ignore)]
    response_sender: mpsc::UnboundedSender<ListResponse>
}


impl NetworkBehaviourEventProcess<FloodsubEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {

    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        
    }
}