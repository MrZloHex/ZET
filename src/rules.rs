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
    pub floodsub:Floodsub,
    pub mdns: TokioMdns,
    #[behaviour(ignore)]
    response_sender: mpsc::UnboundedSender<ListResponse>
}


impl NetworkBehaviourEventProcess<FloodsubEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {

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