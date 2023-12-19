use sui_sdk::SuiClientBuilder;

const COIN_TYPE: &str = "0xc060006111016b8a020ad5b33834984a437aaa7d3c74c18e09a95d48aceab08c::coin::COIN";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let coin_metadata = sui
        .coin_read_api()
        .get_coin_metadata(COIN_TYPE.to_string())
        .await?;
    println!("{:#?}", coin_metadata);

    Ok(())
}
