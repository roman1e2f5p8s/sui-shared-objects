import os
import json
import pandas as pd
import matplotlib.pyplot as plt

FILE = './../results/workspace1/epoch_density_data.json'
with open(FILE, 'r') as f:
    json_ = json.load(f);

BULLSHARK_QUEST_1_START = 85
BULLSHARK_QUEST_1_END = 106

main_df = pd.DataFrame.from_dict(json_['epochs'], orient='index')
main_df.index = main_df.index.astype(int);
interval_df = pd.json_normalize(main_df['avg_interval_data'])

plt.rcParams.update({
    'font.size': 16,
    'text.usetex': True,
    'font.family': 'serif',
    'font.serif': ['Times']
})

# density -----------------------------------------------------------------------
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

ax.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax.axhline(y=0, linestyle=':', linewidth=1, color='black')

ax.plot(main_df['density'], linewidth=2, linestyle='--', marker='o', color='blue')

ax.set_ylabel('Density')
ax.set_xlabel('Epoch')
ax.minorticks_on()
ax.legend()

left_subax = ax.inset_axes([0.27, 0.665, 0.24, 0.19])
left_subax.axhline(y=0, linestyle=':', linewidth=1, color='black')
left_subax.plot(main_df['density'][:19], linewidth=2, linestyle='--', marker='o', color='blue')
left_subax.set_ylabel('Density')
left_subax.set_xlabel('Epoch')
left_subax.ticklabel_format(style='sci', axis='y', scilimits=(0, 0))
# left_subax.set_yticks([0, 2e5, 4e5, 6e5])
#left_subax.set_yticklabels([0, 2, 4, 6])
left_subax.minorticks_on()
ax.indicate_inset_zoom(left_subax)

right_subax = ax.inset_axes([0.77, 0.35, 0.20, 0.20])
# right_subax.axhline(y=0, linestyle=':', linewidth=1, color='black')
right_subax.plot(main_df['density'][BULLSHARK_QUEST_1_START+1:BULLSHARK_QUEST_1_END], linewidth=2, linestyle='--', marker='o', color='blue')
right_subax.ticklabel_format(style='sci', axis='y', scilimits=(0, 0))
right_subax.set_ylabel('Density')
right_subax.set_xlabel('Epoch')
right_subax.minorticks_on()
ax.indicate_inset_zoom(right_subax)

fig.tight_layout()
plt.savefig('./../results/workspace1/density.pdf', format='pdf')
# density -----------------------------------------------------------------------


# transaction number ---------------------------------------------------------
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

ax.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=0.3, color='red', label='Bullshark Quest 1')
ax.axhline(y=0, linestyle=':', linewidth=1, color='black')

ax.plot(main_df['num_txs_total'], linewidth=2, linestyle='--', marker='o', color='blue')

ax.set_ylabel('Number of transactions')
ax.set_xlabel('Epoch')
ax.minorticks_on()
ax.legend(loc='upper left')
# ax.set_yscale('log')

left_subax = ax.inset_axes([0.14, 0.35, 0.35, 0.35])
left_subax.axhline(y=0, linestyle=':', linewidth=1, color='black')
left_subax.plot(main_df['num_txs_total'][:BULLSHARK_QUEST_1_START], linewidth=2, linestyle='--', marker='o', color='blue')
left_subax.set_ylabel('Number of TXs')
left_subax.set_xlabel('Epoch')
left_subax.ticklabel_format(style='sci', axis='y', scilimits=(0, 0))
left_subax.set_yticks([0, 2e5, 4e5, 6e5])
#left_subax.set_yticklabels([0, 2, 4, 6])
left_subax.minorticks_on()
ax.indicate_inset_zoom(left_subax)

right_subax = ax.inset_axes([0.74, 0.35, 0.20, 0.35])
# right_subax.axhline(y=0, linestyle=':', linewidth=1, color='black')
right_subax.plot(main_df['num_txs_total'][BULLSHARK_QUEST_1_END+1:], linewidth=2, linestyle='--', marker='o', color='blue')
# right_subax.set_ylabel('Number of TXs')
right_subax.set_xlabel('Epoch')
right_subax.ticklabel_format(style='sci', axis='y', scilimits=(0, 0))
right_subax.set_xticks([110, 130, 150])
right_subax.minorticks_on()
ax.indicate_inset_zoom(right_subax)

fig.tight_layout()
plt.savefig('./../results/workspace1/tx-number.pdf', format='pdf')
# transaction number ---------------------------------------------------------
