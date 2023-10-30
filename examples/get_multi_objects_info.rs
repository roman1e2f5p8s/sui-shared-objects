use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::ObjectID;
use sui_sdk::rpc_types::SuiObjectDataOptions;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build("https://fullnode.mainnet.sui.io:443")
        .await
        .unwrap();
    println!("\n --- Sui mainnet version: {} --- \n", sui.api_version());

    let object_ids = vec![
        ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000005")?,
        ObjectID::from_str("0x0000000000000000000000000000000000000000000000000000000000000006")?,
        ObjectID::from_str("0xaeab97f96cf9877fee2883315d459552b2b921edc16d7ceac6eab944dd88919c")?,
        ObjectID::from_str("0xc57508ee0d4595e5a8728974a4a93a787d38f339757230d441e895422c07aba9")?,
        ObjectID::from_str("0x23580088ff9f83848f86bc1cbf4964735116027e9cccad11b2dc96f16badab72")?,
        ObjectID::from_str("0x8b55537cb11498721f7bbe2055a9a3e9c947da81765e98d4313c820bdd7aa630")?,
        ObjectID::from_str("0xe288c52bb4d70465267ed1c8f727252a40e865ce9a9d5343d13d40f5e1a1fc46")?,
        ObjectID::from_str("0xcd56eb0efc872d7e27fa9e5e31d70be594ccc4f40a6354521201b0e15971c8de")?,
        ObjectID::from_str("0xb45d92d3ee25147c3235f881e63f7f362f9d6cbdfcba1f120fae6a6c930a1c8c")?,
        ObjectID::from_str("0x1d0975ab0726f2e52384b6ea0f2c94c2dbdad8b004ee6b5ee552a3c789c044f0")?,
        ObjectID::from_str("0xad1c5fd8169160441f0084a138ab726588cf253cc6fab3c32680d2d3164de2db")?,
    ];
    let mut options = SuiObjectDataOptions::new();
    options.show_content = true;

    let objects = sui
        .read_api()
        .multi_get_object_with_options(object_ids, options)
        .await?;
    println!("{:#?}", objects);

    Ok(())
}
