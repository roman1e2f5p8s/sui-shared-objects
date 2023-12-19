use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::rpc_types::{
    SuiObjectResponseQuery,
    SuiObjectDataFilter,
    SuiObjectDataOptions,
};
//use move_core_types::language_storage::StructTag;

const ADDRESS: &str = "0x5ecf33555da8087dd2edb039b6a21bed3f38696a199bfce5f72e4389c28292d0";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let address = SuiAddress::from_str(ADDRESS)?;
    let mut options = SuiObjectDataOptions::new();
    options.show_content = true;
    let mut query = SuiObjectResponseQuery::new(None, Some(options));
    //query.filter = SuiObjectDataFilter::StructType();

    let objects = sui
        .read_api()
        .get_owned_objects(address, Some(query), None, None)
        .await?;
    println!("{:#?}", objects);

    Ok(())
}
