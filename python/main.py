import os
import pandas as pd
import matplotlib.pyplot as plt

NUM_SUBPLOTS = 7
FILE = './../results/plotme.json'
main_df = pd.read_json(FILE, orient='index')
interval_df = pd.json_normalize(main_df['avg_interval_data'])

plt.rcParams.update({'font.size': 14, 'font.family': 'sans-serif'})

fig, (ax1, ax2, ax3, ax4, ax5, ax6, ax7) = plt.subplots(nrows=NUM_SUBPLOTS, ncols=1, figsize=(10, NUM_SUBPLOTS * 7))

ax1.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax1.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax1.plot(main_df['num_txs_total'], linewidth=2, linestyle='--', marker='o', color='orange')
ax1.set_title('Total number of TXs on the Sui mainnet per epoch')
ax1.set_ylabel('TX number')
ax1.minorticks_on()
ax1.legend()

ax2.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax2.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax2.plot(main_df['num_txs_touching_shared_objs'], linewidth=2, linestyle='--', marker='o', color='magenta')
ax2.set_title('Number of TXs touching shared objects on the Sui mainnet per epoch')
ax2.set_ylabel('TX number')
ax2.minorticks_on()
ax2.legend()

ax3.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax3.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax3.plot(main_df['density'] * 100, linewidth=2, linestyle='--', marker='o', color='green')
ax3.set_title('Percentage of TXs involving shared objects on the Sui mainnet per epoch')
ax3.set_ylabel('Density, %')
ax3.minorticks_on()
ax3.legend()

ax4.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax4.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax4.plot(main_df['num_shared_objects_per_epoch'], linewidth=2, linestyle='--', marker='o', color='olive')
ax4.set_title('Number of (unique) shared objects on the Sui mainnet per epoch')
ax4.set_ylabel('Shared object number')
ax4.minorticks_on()
ax4.legend()

ax5.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax5.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax5.plot(main_df['num_shared_objects_total'], linewidth=2, linestyle='--', marker='o', color='blue')
ax5.set_title('Total number of (unique) shared objects on the Sui mainnet')
ax5.set_ylabel('Shared object number')
ax5.minorticks_on()
ax5.legend()

ax6.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax6.axhline(y=1, linestyle='-.', linewidth=1, color='black')
ax6.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
for col in interval_df:
    if 'degree' in col:
        ax6.plot(interval_df[col], linewidth=2, label='Interval: {} checkpoints'.format(col.split('.')[0]))
ax6.set_title('Average number of TXs touching the same shared object within an interval')
ax6.set_ylabel('Avg contention degree')
ax6.minorticks_on()
ax6.legend()

ax7.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax7.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
for col in interval_df:
    if not 'degree' in col:
        ax7.plot(interval_df[col], linewidth=2, label='Interval: {} checkpoints'.format(col.split('.')[0]))
ax7.set_title('Average number of shared objects touched by more than one TX within an interval')
ax7.set_xlabel('Epoch')
ax7.set_ylabel('Avg object touchability')
ax7.minorticks_on()
ax7.legend()

plt.savefig('./../results/figure.png', format='png')
