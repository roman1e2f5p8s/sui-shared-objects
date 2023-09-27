import os
import pandas as pd
import matplotlib.pyplot as plt

FILE = './../results/plotme.json'
main_df = pd.read_json(FILE, orient='index')
interval_df = pd.json_normalize(main_df['contention_degree'][:-1])

plt.rcParams.update({'font.size': 14, 'font.family': 'sans-serif'})

fig, (ax1, ax2, ax3) = plt.subplots(nrows=3, ncols=1, figsize=(10, 21))

ax1.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax1.plot(main_df['num_txs_total'][:-1], linewidth=2, linestyle='--', marker='o', color='orange')
ax1.set_title('Total number of TXs on the Sui mainnet per epoch')
ax1.set_ylabel('TX number')
ax1.legend()

ax2.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax2.plot(main_df['density'][:-1] * 100, linewidth=2, linestyle='--', marker='o', color='green')
ax2.set_title('Percentage of TXs involving shared objects on the Sui mainnet per epoch')
ax2.set_ylabel('Density, %')
ax2.legend()

ax3.axhline(y=1, linestyle='-.', linewidth=1, color='black')
ax3.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
for col in interval_df:
    ax3.plot(interval_df[col], linewidth=2, label='Interval: {} checkpoints'.format(col))
ax3.set_title('Average number of TXs touching the same shared object within an interval')
ax3.set_xlabel('Epoch')
ax3.set_ylabel('Avg contention degree')
ax3.legend()

plt.savefig('./../results/figure.png', format='png')
