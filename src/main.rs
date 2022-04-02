//! Nodes identify each other via their [`PeerId`](crate::PeerId) which is
//! derived from their public key.
use libp2p::{identity, PeerId};
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, dial_opts::DialOpts};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    let transport = libp2p::development_transport(local_key).await?;
    //! A transport in libp2p provides connection-oriented communication channels
    //! The two traits [`Transport`] and [`NetworkBehaviour`] allow us to cleanly
    //! separate _how_ to send bytes from _what_ bytes to send.
    //
    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    //a [`Swarm`] drives both a
    //! [`Transport`] and a [`NetworkBehaviour`] forward, passing commands from the
    //! [`NetworkBehaviour`] to the [`Transport`]
    let mut swarm = Swarm::new(transport,behaviour,local_peer_id);
    // Tell the swarm to listen on all interfaces and a random, OS-assigned
    // port.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
     if let Some(addr) = std::env::args().nth(1) {
         let remote: MultiAddr = addr.parse()?;
         swarm.dial(remote)?;
         println!("Dialed {}", addr)
     }
    Ok(())
}
