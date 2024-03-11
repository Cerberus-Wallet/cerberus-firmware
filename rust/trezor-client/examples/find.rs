fn main() {
    let cerberuss = cerberus_client::find_devices(false);
    println!("Found {} devices: ", cerberuss.len());
    for t in cerberuss.into_iter() {
        println!("- {}", t);
        {
            let mut client = t.connect().unwrap();
            println!("{:?}", client.initialize(None).unwrap());
        }
    }
}
