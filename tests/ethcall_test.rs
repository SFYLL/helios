use env_logger::Env;
use helios::{
    client::{Client, ClientBuilder, FileDB},
    config::networks::Network,
    types::{BlockTag, CallOpts},
};use std::time::Duration;
use std::{env, path::PathBuf};
use ethers::types::{U256, U64, Address};

#[tokio::test]
async fn ethcall_test() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Client Configuration
    let api_key = env::var("MAINNET_RPC_URL").expect("MAINNET_RPC_URL env variable missing");
    let checkpoint = "0x4d9b87a319c52e54068b7727a93dd3d52b83f7336ed93707bcdf7b37aefce700";
    let consensus_rpc = "https://www.lightclientdata.org";
    let data_dir = "/tmp/helios";
    log::info!("Using consensus RPC URL: {}", consensus_rpc);

    // Instantiate Client
    let mut client: Client<FileDB> = ClientBuilder::new()
        .network(Network::MAINNET)
        .consensus_rpc(consensus_rpc)
        .execution_rpc(&api_key)
        .checkpoint(checkpoint)
        .load_external_fallback()
        .data_dir(PathBuf::from(data_dir))
        .build().unwrap();

    log::info!(
        "Built client on \"{}\" with external checkpoint fallbacks",
        Network::MAINNET
    );

    client.start().await.unwrap();

    // Wait for syncing
    std::thread::sleep(Duration::from_secs(5));
    let block = BlockTag::Latest;

    let calls= ["0x70a082310000000000000000000000009527465642a7015738ef24201eec1644f3755670", "0x70a082310000000000000000000000000f366a411dc9f8a1611cad614d8f451436fc4f4b", "0x70a08231000000000000000000000000630276c20064545c06360bbd3ef48025abe3316a", "0x70a08231000000000000000000000000be6e784ad98581be1077ddf630205ac30ce8128b", "0x70a08231000000000000000000000000a48aa5c696357f29a187fb408f1a5c9ecab445c5", "0x70a082310000000000000000000000009527465642a7015738ef24201eec1644f3755670", "0x70a082310000000000000000000000000f366a411dc9f8a1611cad614d8f451436fc4f4b", "0x70a08231000000000000000000000000630276c20064545c06360bbd3ef48025abe3316a", "0x70a08231000000000000000000000000be6e784ad98581be1077ddf630205ac30ce8128b", "0x70a08231000000000000000000000000a48aa5c696357f29a187fb408f1a5c9ecab445c5", "0x70a082310000000000000000000000009527465642a7015738ef24201eec1644f3755670", "0x70a082310000000000000000000000000f366a411dc9f8a1611cad614d8f451436fc4f4b", "0x70a08231000000000000000000000000630276c20064545c06360bbd3ef48025abe3316a", "0x70a08231000000000000000000000000be6e784ad98581be1077ddf630205ac30ce8128b", "0x70a08231000000000000000000000000a48aa5c696357f29a187fb408f1a5c9ecab445c5", "0x70a082310000000000000000000000009527465642a7015738ef24201eec1644f3755670", "0x70a082310000000000000000000000000f366a411dc9f8a1611cad614d8f451436fc4f4b", "0x70a08231000000000000000000000000630276c20064545c06360bbd3ef48025abe3316a", "0x70a08231000000000000000000000000be6e784ad98581be1077ddf630205ac30ce8128b", "0x70a08231000000000000000000000000a48aa5c696357f29a187fb408f1a5c9ecab445c5", "0x70a082310000000000000000000000009527465642a7015738ef24201eec1644f3755670", "0x70a082310000000000000000000000000f366a411dc9f8a1611cad614d8f451436fc4f4b", "0x70a08231000000000000000000000000630276c20064545c06360bbd3ef48025abe3316a", "0x70a08231000000000000000000000000be6e784ad98581be1077ddf630205ac30ce8128b", "0x70a08231000000000000000000000000a48aa5c696357f29a187fb408f1a5c9ecab445c5"];
    let addresses= ["0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xdAC17F958D2ee523a2206206994597C13D831ec7", "0xD46bA6D942050d489DBd938a2C909A5d5039A161", "0xD46bA6D942050d489DBd938a2C909A5d5039A161", "0xD46bA6D942050d489DBd938a2C909A5d5039A161", "0xD46bA6D942050d489DBd938a2C909A5d5039A161", "0xD46bA6D942050d489DBd938a2C909A5d5039A161", "0x6B175474E89094C44Da98b954EedeAC495271d0F", "0x6B175474E89094C44Da98b954EedeAC495271d0F", "0x6B175474E89094C44Da98b954EedeAC495271d0F", "0x6B175474E89094C44Da98b954EedeAC495271d0F", "0x6B175474E89094C44Da98b954EedeAC495271d0F", "0x92D6C1e31e14520e676a687F0a93788B716BEff5", "0x92D6C1e31e14520e676a687F0a93788B716BEff5", "0x92D6C1e31e14520e676a687F0a93788B716BEff5", "0x92D6C1e31e14520e676a687F0a93788B716BEff5", "0x92D6C1e31e14520e676a687F0a93788B716BEff5"];
    let mut results = vec![];
    for (index, item) in calls.iter().enumerate() {
        let call_opts = CallOpts {
            from: Some("0xBE0eB53F46cd790Cd13851d5EFf43D12404d33E8".parse::<Address>().unwrap()),
            to: Some(addresses[index].parse::<Address>().unwrap()),
            gas: Some(U256::from(U64::MAX.as_u64())),
            gas_price: None,
            value: None,
            data: Some(common::utils::hex_str_to_bytes(item).unwrap()),
        };
  
        log::info!("Calling helios client with args {call_opts:?}");
        let result = client.call(&call_opts, block).await.unwrap();
        log::info!("result{result:?}");
        results.push(result);
    }

    assert!(results.len() == calls.len());

}
