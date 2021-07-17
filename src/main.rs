#![allow(non_snake_case)]

mod user_node;
use user_node::user::User;



use libp2p::{NetworkBehaviour, PeerId, Swarm, Transport, core::upgrade, floodsub::{Floodsub, FloodsubEvent, Topic}, identity, mdns::{TokioMdns}, mplex, noise::{Keypair, NoiseConfig, X25519Spec}, swarm::{NetworkBehaviourEventProcess, SwarmBuilder}, tcp::TokioTcpConfig};
use tokio::{
    fs,
    io::AsyncBufRead,
    sync::mpsc
};



#[tokio::main]
async fn main() {
    let username = if let Some(uname) = std::env::args().nth(1) {
        println!("Creating user {}", uname);
        uname
    } else {
        "user".to_string()
    };

    let Me = User::new(username, identity::Keypair::generate_ed25519(), Topic::new("ZET"));

    let (response_sender, mut response_rcv) = mpsc::unbounded_channel();


    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&keys)
        .expect("Can create  auth keys");

    let transport = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let mut behavior = MessageBehaviour {
        floodsub: Floodsub::new(peer_id.clone()),
        mdns: TokioMdns::new().expect("Can create mdns"),
        response_sender,
    };

    behavior.floodsub.subscribe(topic.clone());


    let mut swarm = SwarmBuilder::new(transport, behavior, peer_id.clone())
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    Swarm::listen_on(
        &mut swarm,
        "/ip4/0.0.0.0/tcp/0"
            .parse()
            .expect("can get local socket")
    )
    .expect("swarm can be started");

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();


    loop {
        let may_event = {
            tokio::select! {
                line = stdin.next_line() => Some(EventType::Input(line.expect("can get line").expect("acn read line from stdin"))),
                event = swarm.next_line => {
                    println!("Unhandled Swarm Event: {:?}", event);
                    None
                },
                response = response_rcv.recv() => Some(EventType::Response(response.expect("response exists")))
            }
        };

    //     if let Some(event) = may_event {
    //         match event {
    //             EventType::Response(resp) => {
    //                 let json = serde_json::to_string(&resp).expect("can jsonify response");
    //                 swarm.floodsub.publish(TOPIC.clone(), json.as_bytes());
    //             }
    //             EventType::Input(line) => match line.as_str() {
    //                 "ls p" => handle_list_peers(&mut swarm).await,
    //                 cmd if cmd.starts_with("ls r") => handle_list_recipes(cmd, &mut swarm).await,
    //                 cmd if cmd.starts_with("publish r") => handle_publish_recipe(cmd).await,
    //                 _ => println!("unknown command"),
    //             },
    //         }
    //     };
    // };


}
