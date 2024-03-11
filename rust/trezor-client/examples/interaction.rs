use std::io;

use bitcoin::{bip32, network::Network, Address};
use cerberus_client::{Error, CerberusMessage, CerberusResponse};

fn handle_interaction<T, R: CerberusMessage>(resp: CerberusResponse<T, R>) -> Result<T, Error> {
    match resp {
        CerberusResponse::Ok(res) => Ok(res),
        CerberusResponse::Failure(_) => resp.ok(), // assering ok() returns the failure error
        CerberusResponse::ButtonRequest(req) => handle_interaction(req.ack()?),
        CerberusResponse::PinMatrixRequest(req) => {
            println!("Enter PIN");
            let mut pin = String::new();
            if io::stdin().read_line(&mut pin).unwrap() != 5 {
                println!("must enter pin, received: {}", pin);
            }
            // trim newline
            handle_interaction(req.ack_pin(pin[..4].to_owned())?)
        }
        CerberusResponse::PassphraseRequest(req) => {
            println!("Enter passphrase");
            let mut pass = String::new();
            io::stdin().read_line(&mut pass).unwrap();
            // trim newline
            handle_interaction(req.ack_passphrase(pass[..pass.len() - 1].to_owned())?)
        }
    }
}

fn do_main() -> Result<(), cerberus_client::Error> {
    // init with debugging
    let mut cerberus = cerberus_client::unique(true)?;
    cerberus.init_device(None)?;

    let xpub = handle_interaction(
        cerberus.get_public_key(
            &vec![
                bip32::ChildNumber::from_hardened_idx(0).unwrap(),
                bip32::ChildNumber::from_hardened_idx(0).unwrap(),
                bip32::ChildNumber::from_hardened_idx(0).unwrap(),
            ]
            .into(),
            cerberus_client::protos::InputScriptType::SPENDADDRESS,
            Network::Testnet,
            true,
        )?,
    )?;
    println!("{}", xpub);
    println!("{:?}", xpub);
    println!("{}", Address::p2pkh(&xpub.to_pub(), Network::Testnet));

    Ok(())
}

fn main() {
    do_main().unwrap()
}
