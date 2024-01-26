import os
import json
import pandas as pd
import matplotlib.pyplot as plt

NUM_SUBPLOTS = 7
FILE = './../results/workspace1/epoch_density_data.json'
with open(FILE, 'r') as f:
    json_ = json.load(f);

BULLSHARK_QUEST_1_START = 85
BULLSHARK_QUEST_1_END = 106
BULLSHARK_QUEST_2_START = 107
BULLSHARK_QUEST_2_END = 146
BULLSHARK_QUEST_3_START = 183
BULLSHARK_QUEST_3_END = 211
WINTER_QUEST_START = 250
WINTER_QUEST_END = list(json_['epochs'].keys())[-1] # TODO


main_df = pd.DataFrame.from_dict(json_['epochs'], orient='index')
main_df.index = main_df.index.astype(int);
interval_df = pd.json_normalize(main_df['avg_interval_data'])

plt.rcParams.update({'font.size': 14, 'font.family': 'sans-serif'})

fig, (ax1, ax2, ax3, ax4, ax5, ax6, ax7) = plt.subplots(nrows=NUM_SUBPLOTS, ncols=1, figsize=(10, NUM_SUBPLOTS * 7))

print('Total number of scanned TXs: {}'.format(main_df['num_txs_total'].sum()))

ax1.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax1.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=0.3, color='green', label='Bullshark Quest 2')
ax1.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=0.3, color='blue', label='Bullshark Quest 3')
ax1.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=0.3, color='cyan', label='Winter Quest')
ax1.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax1.plot(main_df['num_txs_total'], linewidth=2, linestyle='--', marker='o', color='orange')
ax1.set_title('Total number of TXs on the Sui mainnet per epoch')
ax1.set_ylabel('TX number')
ax1.minorticks_on()
ax1.legend()

ax2.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax2.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=0.3, color='green', label='Bullshark Quest 2')
ax2.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=0.3, color='blue', label='Bullshark Quest 3')
ax2.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=0.3, color='cyan', label='Winter Quest')
ax2.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax2.plot(main_df['num_txs_touching_shared_objs'], linewidth=2, linestyle='--', marker='o', color='magenta')
ax2.set_title('Number of TXs touching shared objects on the Sui mainnet per epoch')
ax2.set_ylabel('TX number')
ax2.minorticks_on()
ax2.legend()

ax3.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax3.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=0.3, color='green', label='Bullshark Quest 2')
ax3.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=0.3, color='blue', label='Bullshark Quest 3')
ax3.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=0.3, color='cyan', label='Winter Quest')
ax3.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax3.plot(main_df['density'] * 100, linewidth=2, linestyle='--', marker='o', color='green')
ax3.set_title('Percentage of TXs involving shared objects on the Sui mainnet per epoch')
ax3.set_ylabel('Density, %')
ax3.minorticks_on()
ax3.legend()

ax4.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax4.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=0.3, color='green', label='Bullshark Quest 2')
ax4.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=0.3, color='blue', label='Bullshark Quest 3')
ax4.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=0.3, color='cyan', label='Winter Quest')
ax4.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax4.plot(main_df['num_shared_objects_per_epoch'], linewidth=2, linestyle='--', marker='o', color='olive')
ax4.set_title('Number of (unique) shared objects on the Sui mainnet per epoch')
ax4.set_ylabel('Shared object number')
ax4.minorticks_on()
ax4.legend()

ax5.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax5.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=0.3, color='green', label='Bullshark Quest 2')
ax5.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=0.3, color='blue', label='Bullshark Quest 3')
ax5.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=0.3, color='cyan', label='Winter Quest')
ax5.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax5.plot(main_df['num_shared_objects_total'], linewidth=2, linestyle='--', marker='o', color='blue')
ax5.set_title('Total number of (unique) shared objects on the Sui mainnet')
ax5.set_ylabel('Shared object number')
ax5.minorticks_on()
ax5.legend()

ax6.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax6.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=0.3, color='green', label='Bullshark Quest 2')
ax6.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=0.3, color='blue', label='Bullshark Quest 3')
ax6.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=0.3, color='cyan', label='Winter Quest')
ax6.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax6.axhline(y=1, linestyle='-.', linewidth=1, color='black')
for col in interval_df:
    if 'degree' in col:
        ax6.plot(interval_df[col], linewidth=2, label='Interval: {} checkpoints'.format(col.split('.')[0]))
ax6.set_title('Average number of TXs touching the same shared object within an interval')
ax6.set_ylabel('Avg contention degree')
ax6.minorticks_on()
ax6.legend()

ax7.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax7.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=0.3, color='green', label='Bullshark Quest 2')
ax7.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=0.3, color='blue', label='Bullshark Quest 3')
ax7.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=0.3, color='cyan', label='Winter Quest')
ax7.axhline(y=0, linestyle=':', linewidth=1, color='black')
for col in interval_df:
    if not 'degree' in col:
        ax7.plot(interval_df[col], linewidth=2, label='Interval: {} checkpoints'.format(col.split('.')[0]))
ax7.set_title('Average number of shared objects touched by more than one TX within an interval')
ax7.set_xlabel('Epoch')
ax7.set_ylabel('Avg object touchability')
ax7.minorticks_on()
ax7.legend()

fig.tight_layout()
plt.savefig('./../results/workspace1/figure.png', format='png')
