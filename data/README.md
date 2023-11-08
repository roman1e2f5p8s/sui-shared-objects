After running `query_txs` for an entire epoch, this folder will contain a workspace folder with
data files, one file per epoch, containing information of interest about Sui transactions for that epoch.
The default workspace is `workspace1` and it can be changed by using the command line argument `--workspace`
for `query_txs`.

If you an IF member, you should have access to the data produced by `query_txs` for several epochs
on shared Google Drive [TODO], from where it can be downloaded and placed in the `workspace1` directory 
in this folder.

A data file will have the name indicating the epoch number and its boundaries expressed
in checkpoints (for example, `epoch=021_1584197-1668109.json`) and contain the following information:
- `network` is a string that indicates which Sui network type (one of `"Mainnet"`, `"Devnet"`, `"Testnet"`)
was used to query transactions by `query_txs`;
- `version` is a string that indicates the version of the Sui network `network`; for example, `"1.12.2"`;
- `epoch` is an unsigned integer that indicates which epoch transactions were queried for; for example, `21`;
- `start_checkpoint` is an unsigned integer that indicates the checkpoint at which the epoch started; for example, `1584197`;
- `end_checkpoint` is an unsigned integer that indicates the checkpoint at which the epoch ended; for example, `1668109`;
- `last_cursor` is a string that indicates the digest of a transaction at which the query ended; for example,
`"52WSZ3EmWq9v8TmcSKhTdJgdG1ZrHB7YMmvSyvshywDN"`; this might be the last transaction in the epoch, like in this
example, or a transaction at which the query stopped due to dropped connection, in which case this digest is
used to continue the query of the rest of transactions for the epoch;
- `num_txs_in_epoch` is an unsigned integer that indicates the number of transactions in the epoch;
for example, ` 675133`; this number is taken from the [Sui Explorer](https://suiexplorer.com/recent?tab=epochs&network=mainnet)
and it is saved in the [EPOCH_TO_CHECKPOINTS.json](https://github.com/roman1e2f5p8s/sui-shared-object-density/blob/main/results/EPOCH_TO_CHECKPOINTS.json) data file;
- `num_txs_scanned` is an unsigned integer that indicates the number of scanned transactions in the epoch;
for example, `675133`; this number must match with `num_txs_in_epoch` meaning that indeed all transactions in the 
epoch were scanned, otherwise (in the case of dropped connection), the query must be continued to scan the rest of transactions in the epoch;
- `num_txs_touching_0_shared_objs` is an unsigned integer that indicates the number of transactions in the epoch
that have no shared objects in their inputs; for example, `349769`;
- `num_txs_touching_0_objs` is an unsigned integer that indicates the number of transactions in the epoch
that have no objects in their inputs; for example, `85406`;
- `checkpoints` is a mapping from checkpoint (also knowns as sequence number in Sui) being a string to the
data of interest about that checkpoint; this data includes:
    - `num_txs_total` is an unsigned integer that indicates the total number of transactions in that checkpoint;
    for example, `15`;
    - `num_txs_touching_shared_objs` is an unsigned integer that indicates the total number of transactions 
    that have at least one shared object in their inputs in that checkpoint; for example, `8`;
    - `shared_objects` is a mapping from shared object ID being a string to a set of transactions
    that have that shared object in their inputs; for example, `"0x0000000000000000000000000000000000000000000000000000000000000006": 
    {"DKQDGLoTsY97gbv2mADmiFz8r7mApncKYn1hCqatKeSR": false}`, where:
        - `0x0000000000000000000000000000000000000000000000000000000000000006` is the shared object ID;
        - `"DKQDGLoTsY97gbv2mADmiFz8r7mApncKYn1hCqatKeSR"` is a string that indicates the digest of a transaction that
        touched that shared object;
        - `false` is a boolean that indicates that the shared object was not passed by a mutable reference in that transaction.

The purpose of this data file is:
- TODO

A snippet of this data file structure looks as follows:
```json
{
  "network": "Mainnet",
  "version": "1.12.2",
  "epoch": 21,
  "start_checkpoint": 1584197,
  "end_checkpoint": 1668109,
  "last_cursor": "52WSZ3EmWq9v8TmcSKhTdJgdG1ZrHB7YMmvSyvshywDN",
  "num_txs_in_epoch": 675133,
  "num_txs_scanned": 675133,
  "num_txs_touching_0_shared_objs": 349769,
  "num_txs_touching_0_objs": 85406,
  "checkpoints": {
    "1584197": {
      "num_txs_total": 1,
      "num_txs_touching_shared_objs": 0,
      "shared_objects": {}
    },
    "1584198": {
      "num_txs_total": 15,
      "num_txs_touching_shared_objs": 8,
      "shared_objects": {
        "0x0000000000000000000000000000000000000000000000000000000000000006": {
          "DKQDGLoTsY97gbv2mADmiFz8r7mApncKYn1hCqatKeSR": false
        },
        "0x09e24b156b08e7bc5272f9b731e93b80b458f0b79a5195eb81a471d514f1b1b8": {
          "7SGrzSGFfR11UKdTgQKMraZ3DFPZSCWmuA9qqesk6zT9": true,
          "8Xg9JPuwZaiv5m43QuWUyaZqPDrvxzHJNsgGQVRPtdRr": true,
          "DnrSH8k6XYDaMmnqDm5fLgiARGrNkpxVy42CuwFxo8yR": true
        },
        "0x3083e3d751360c9084ba33f6d9e1ad38fb2a11cffc151f2ee4a5c03da61fb1e2": {
          "DKQDGLoTsY97gbv2mADmiFz8r7mApncKYn1hCqatKeSR": true
        },
        "0x4a8e6a4634e3dedae00ffe9f065351664ba32d7e9c2d26221a666ca380ea68b9": {
          "4MY3VWLce6kUWFa28BDNXBnEGTBBEhfQVuXN4vKBzbNX": true
        },
        "0x64168ef7953cbdb3cf0b3e4f13301061740d2b1d015900e1ae025d31515ad830": {
          "2pUbL3pSrsxNvJ42fgDETFpW9t58wHhHWbKFppK3vkeu": false,
          "5fWWr9TnYw3c8AL63845pFyXYXxBUA91TUqt4Q2uNNU1": false
        },
        "0x6bdeb62b036f5f9c3f0587a0480e6dd75cb4e758660cf30b542c031fa394bb83": {
          "DnrSH8k6XYDaMmnqDm5fLgiARGrNkpxVy42CuwFxo8yR": true
        },
        "0xb2b140b2841329320b66f92373a2683af7f9066472233effab03755270bcf65f": {
          "AWeevWRKWPK59Uisv3D32totUe9XK7DeFAvLyb744BmJ": true
        },
        "0xbf471e4f38f76ed88f18e44f3dce2c27e03c1857d51ea17cd9b230b6d69b4bc1": {
          "DnrSH8k6XYDaMmnqDm5fLgiARGrNkpxVy42CuwFxo8yR": true
        },
        "0xf0c8e045496bddbef8261cd816f21f84368adafee230fa909a2403c473bdbee7": {
          "2pUbL3pSrsxNvJ42fgDETFpW9t58wHhHWbKFppK3vkeu": true,
          "5fWWr9TnYw3c8AL63845pFyXYXxBUA91TUqt4Q2uNNU1": true
        }
      }
    },
  }
}
```
