use sui_sdk::rpc_types::SuiCallArg;
use sui_sdk::rpc_types::SuiObjectArg;
use sui_sdk::rpc_types::SuiTransactionBlock;
use sui_sdk::rpc_types::SuiTransactionBlockData;
use sui_sdk::rpc_types::SuiTransactionBlockKind;

use crate::types::{TxInfo, SharedObjInfo};


// print type of variable
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


// Given Option<sui_json_rpc_types::sui_transaction::SuiTransactionBlock>
// for TX, return its inputs
pub fn process_tx_inputs(tx_block: &Option<SuiTransactionBlock>) -> TxInfo {
    // `tx_block` should have structure like this:
    // Some(SuiTransactionBlock {
    //  data: V1(SuiTransactionBlockDataV1 { 
    //      transaction: ProgrammableTransaction(SuiProgrammableTransactionBlock { 
    //      inputs: [
    // So, we need to unwrap Some, and get the `data` field of SuiTransactionBlock.
    // Then, we access the V1 variant of the SuiTransactionBlockData enum.
    // There is only one variant, so we don't need `if let`
    let SuiTransactionBlockData::V1(tx_data_v1) = &tx_block.as_ref().unwrap().data;
    // let SuiTransactionBlockData::V1(tx_data_v1) = match &tx_block.as_ref() {
    //     Ok(block) => {
    //         block.data
    //     }
    //     Err(error) => {
    //         println!("\n  {}: {:?}", "ERROR", error);
    //     }
    // };

    // Now, get the `transaction` field of the SuiTransactionBlockDataV1 struct,
    // then access the ProgrammableTransaction variant of 
    // the SuiTransactionBlockKind enum
    if let SuiTransactionBlockKind::ProgrammableTransaction(prog_tx) = &tx_data_v1.transaction {
        // to count the number of shared mutable objects
        let mut count = 0;
        let mut shared_objects: Vec<SharedObjInfo> = Vec::new();

        for input in prog_tx.inputs.iter() {
            // input has type of sui_sdk::rpc_types::SuiCallArg;
            // the sui_sdk::rpc_types::SuiCallArg enum has two variants:
            // Object and Pure. We need only Objects.
            if let SuiCallArg::Object(obj) = input {
                // obj has type of sui_sdk::rpc_types::SuiObjectArg;
                // sui_sdk::rpc_types::SuiObjectArg enum has two variants:
                // ImmOrOwnedObject and SharedObject. We need only SharedObject
                if let SuiObjectArg::SharedObject{object_id, mutable, ..} = obj {
                    count = count + 1;
                    shared_objects.push(SharedObjInfo {
                        id: object_id.to_string(),
                        mutable: *mutable
                    })
                }
            }
        }
        return TxInfo {
            num_total: prog_tx.inputs.len(),
            num_shared: count,
            shared_objects: shared_objects
        };
    }
    TxInfo {
        num_total: 0,
        num_shared: 0,
        shared_objects: Vec::new()
    }
}


pub fn get_imm_or_owned_input_objects(tx_block: &Option<SuiTransactionBlock>) -> Vec<String> {
    let SuiTransactionBlockData::V1(tx_data_v1) = &tx_block.as_ref().unwrap().data;

    if let SuiTransactionBlockKind::ProgrammableTransaction(prog_tx) = &tx_data_v1.transaction {
        let mut imm_or_owned_objects: Vec<String> = Vec::new();

        for input in prog_tx.inputs.iter() {
            if let SuiCallArg::Object(obj) = input {
                if let SuiObjectArg::ImmOrOwnedObject {object_id, ..} = obj {
                    imm_or_owned_objects.push(object_id.to_string());
                }
            }
        }
        return imm_or_owned_objects;
    }
    Vec::new()
}
