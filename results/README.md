Data file [EPOCH_TO_CHECKPOINTS.json](https://github.com/roman1e2f5p8s/sui-shared-object-density/blob/main/results/EPOCH_TO_CHECKPOINTS.json) 
contains the following information about Sui epochs:
- `start_checkpoint` is the checkpoint at which an epoch started;
- `end_checkpoint` is the checkpoint at which the epoch ended;
- `tx_number` is the total number of transactions in that epoch.

The purpose of this data file is twofold:
1. To have a mapping from epoch to its boundaries expressed in checkpoints:
 `start_checkpoint` and `end_checkpoint`.
2. To check whether `query_txs` scanned all the transactions for a given epoch
 by comparing the number of scanned transactions with the number of transactions seen
on the [Sui Explorer](https://suiexplorer.com/recent?tab=epochs&network=mainnet).

> :warning: This file is not machine-generated. It is human-generated from the 
[Sui Explorer](https://suiexplorer.com/recent?tab=epochs&network=mainnet).

The data file has the following structure:
```json
{
  "epochs": {
    "0": {
      "start_checkpoint": 0,
      "end_checkpoint": 9769,
      "tx_number": 9771
    },
}
```
