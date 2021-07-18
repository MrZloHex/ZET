use libp2p::{NetworkBehaviour, floodsub::{Floodsub, FloodsubEvent}, mdns::{MdnsEvent, TokioMdns}, swarm::NetworkBehaviourEventProcess};
use tokio::sync::mpsc;

pub enum MessageType {
    Invite,
    Transfer,
    Request,
    Response
}

pub enum EventType {
    Response(Response),
    Input(String)
}


pub struct Request;

pub struct Response {
    pub data: Message,
    pub receiver: String
}

pub struct Message {
    mes_type: MessageType,
    pub data: String
}

#[derive(NetworkBehaviour)]
pub struct MessageBehaviour {
    pub floodsub:Floodsub,
    pub mdns: TokioMdns,
    #[behaviour(ignore)]
    pub response_sender: mpsc::UnboundedSender<Response>
}


impl NetworkBehaviourEventProcess<FloodsubEvent> for MessageBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(message) => {
                println!("Received {}", String::from_utf8(message.data.clone()).unwrap());
                respond_with_public_recipes(
                    self.response_sender.clone(),
            message.source.to_string(),
                );
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

fn respond_with_public_recipes(sender: mpsc::UnboundedSender<Response>, receiver: String) {
    // info!("sender: {:?}, receiver {}", sender, receiver);
    println!("RESONDING");
    tokio::spawn(
        async move {
            let response_mes = Message { mes_type: MessageType::Response, data: format!("Hi, {}", receiver.clone()) };
            let response = Response {   
                data: response_mes,
                receiver
            };
            if let Err(e) = sender.send(response) {
                println!("error sending response via channel, {}", e);
            };
        }
    );
}