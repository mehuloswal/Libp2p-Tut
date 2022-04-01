//! Nodes identify each other via their [`PeerId`](crate::PeerId) which is
//! derived from their public key.
use libp2p::{identity, PeerId};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);
    Ok(())
}
