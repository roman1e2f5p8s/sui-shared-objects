use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::rpc_types::SuiObjectDataOptions;

const OBJECT_ID: &str = "0x544a93ef9dc62d46c28a9b053b93b3bfae8969543500575e658d6363a944ac47";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let object_id = ObjectID::from_str(OBJECT_ID)?;
    let mut options = SuiObjectDataOptions::new();
    //options.show_type = true;
    options.show_owner = true;
    options.show_content = true;

    let object = sui
        .read_api()
        .get_object_with_options(object_id, options)
        .await?;
    println!("{:#?}", object);

    Ok(())
}
