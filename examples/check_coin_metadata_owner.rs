use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::rpc_types::SuiObjectDataOptions;

const ADDRESS: &str = "0x544a93ef9dc62d46c28a9b053b93b3bfae8969543500575e658d6363a944ac47";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let mut options = SuiObjectDataOptions::new();
    options.show_owner = true;

    let coins = sui
        .coin_read_api()
        .get_all_coins(SuiAddress::from_str(ADDRESS)?, None, None)
        .await?;

    for coin in coins.data {
        let coin_metadata = sui
            .coin_read_api()
            .get_coin_metadata(coin.coin_type)
            .await?;
        if !coin_metadata.is_none() {
            let object = sui
                .read_api()
                .get_object_with_options(coin_metadata.clone().unwrap().id.unwrap(), options.clone())
                .await?;
            println!("{}: {}", coin_metadata.unwrap().id.unwrap(), object.data.unwrap().owner.unwrap());
        }
    }

    Ok(())
}
