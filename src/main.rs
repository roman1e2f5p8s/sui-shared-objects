use std::str::FromStr;
// use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::TransactionDigest;
use sui_sdk::rpc_types::SuiTransactionBlockResponseOptions;
use sui_sdk::rpc_types::SuiTransactionBlockData;
use sui_sdk::rpc_types::SuiTransactionBlockKind;
use sui_sdk::rpc_types::SuiCallArg;
use sui_sdk::rpc_types::SuiObjectArg;

// fn print_type_of<T>(_: &T) {
// println!("{}", std::any::type_name::<T>())
// }

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let sui = SuiClientBuilder::default().build(
        "https://fullnode.mainnet.sui.io:443",
    ).await.unwrap();
    println!("Sui mainnet version: {}", sui.api_version());

    // let address =
    // SuiAddress::from_str("0xd71a30740d758c11f97e4743769a729b5188e2266454cdaaaffe5728bccb29b2")?;
    // let objects = sui.read_api().get_owned_objects(address, None, None, None).await?;
    // println!("{:?}", objects.data);

    let tx_digest = TransactionDigest::from_str("E2hjN5qEWHFuW1wYSL2KqFpWKbfgYPGoxUHzxYgbkzLV")?;
    let tx_options = SuiTransactionBlockResponseOptions::new().with_input();
    let tx = sui
        .read_api()
        .get_transaction_with_options(tx_digest, tx_options)
        .await?;

    let SuiTransactionBlockData::V1(tx_data_v1) = tx.transaction.unwrap().data;
    if let SuiTransactionBlockKind::ProgrammableTransaction(prog_tx) = tx_data_v1.transaction {
        let inputs = prog_tx.inputs;
        println!("Number of inputs: {}", inputs.len());

        let mut count = 0;
        for input in inputs.iter() {
            // input has type of sui_sdk::rpc_types::SuiCallArg;
            // sui_sdk::rpc_types::SuiCallArg enum has two variants:
            // Object and Pure. We need only Objects
            // TODO: check with someone diffs between ImmObj and immutable SharedObj

            if let SuiCallArg::Object(obj) = input {
                // obj has type of sui_sdk::rpc_types::SuiObjectArg;
                // sui_sdk::rpc_types::SuiObjectArg enum has two variants:
                // ImmOrOwnedObject and SharedObject. We need only mutable SharedObject
                
                if let SuiObjectArg::SharedObject{mutable, ..} = obj {
                    if *mutable == true { count = count + 1 };
                }
            }
        }
        println!("Number of shared objects touched by TX: {}", count);
    }

    Ok(())
}
