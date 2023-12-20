use std::str::FromStr;

use sui_sdk::SuiClientBuilder;
use sui_sdk::types::base_types::SuiAddress;
use sui_sdk::rpc_types::{
    SuiObjectResponseQuery,
    SuiObjectDataFilter,
    SuiObjectDataOptions,
};
use sui_sdk::rpc_types::SuiParsedData;
//use move_core_types::language_storage::StructTag;

const ADDRESS: &str = "0xbd98eff8cb12fbcb269a334137c00c693d704ce878d8e1fed640acedb235254f";

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
    let query = SuiObjectResponseQuery::new(None, Some(options));
    //query.filter = SuiObjectDataFilter::StructType();

    let mut cursor = None;

    while {
        let objects = sui
            .read_api()
            .get_owned_objects(address, Some(query.clone()), cursor, None)
            .await?;

        for object in &objects.data {
            let sui_obj_data = object.data.as_ref().unwrap();

            if let SuiParsedData::MoveObject(sui_parsed_move_object) = sui_obj_data.content.as_ref().unwrap() {
                let object_id = sui_obj_data
                    .object_id
                    .to_string();
                let address = sui_parsed_move_object
                    .type_
                    .address
                    .to_string();
                let type_ = sui_parsed_move_object
                    .type_
                    .module
                    .to_string() + &String::from("::") + &sui_parsed_move_object.type_.name.to_string();
                println!("{}: {}::{}", object_id, address, type_);
            }
        }

        cursor = objects.next_cursor;
        objects.has_next_page
    } {}

    Ok(())
}
