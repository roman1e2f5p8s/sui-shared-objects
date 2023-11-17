use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::ObjectID;

const PKG_ID: &str = "0xf85f7a7842c597e272c01d53866f30077d814eb587a56f1a52e58538c4739f80";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let pkg_id = ObjectID::from_str(PKG_ID)?;

    let pkg = sui
        .read_api()
        .get_normalized_move_modules_by_package(pkg_id)
        .await?;
    println!("{:#?}", pkg);

    Ok(())
}
