use aws_sdk_ec2::{Client, Error};

// Create EC2 client
pub async fn ec2client() -> Result<Client, Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    Ok(client)
}

/* -----------------------------
    INSTANCE FNXNS
-------------------------------- */
// List all instances
pub async fn list_instances(client: &Client) -> Result<(), Error> {
    let resp = client
        .describe_instances()
        .send()
        .await?;
    for reservation in resp.reservations() {
        for instance in reservation.instances() {
            println!("Instance ID: {}", instance.instance_id().unwrap());
            println!(
                "State:       {:?}",
                instance.state().unwrap().name().unwrap()
            );
            println!();
        }
    }
    Ok(())
}

// Get instance public dns
pub async fn get_instance_dns(client: &Client, id: &str) -> Result<String, Error> {
    let resp = client
        .describe_instances()
        .instance_ids(id)
        .send()
        .await?;
    let reservation = resp.reservations().get(0).unwrap();
    let instance = reservation.instances().get(0).unwrap();
    let public_dns = instance.public_dns_name().unwrap();
    Ok(public_dns.to_string())
}
// Start an instance
pub async fn start_instance(client: &Client, id: &str) -> Result<(), Error> {
    let _resp = client
        .start_instances()
        .instance_ids(id)
        .send()
        .await?;
    println!("Instance {} started.", id);
    let dns = get_instance_dns(client, id).await?;
    println!("Instance launched at endpoint: https://{}", dns);
    Ok(())
}

// Stop an instance
pub async fn stop_instance(client: &Client, id: &str) -> Result<(), Error> {
    let _resp = client
        .stop_instances()
        .instance_ids(id)
        .send()
        .await?;
    println!("Instance {} stopped.", id);
    Ok(())
}