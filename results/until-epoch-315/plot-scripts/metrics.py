import os
import json
import pandas as pd
import matplotlib.pyplot as plt

FILE = os.path.join(os.pardir, 'epoch_density_data.json')
with open(FILE, 'r') as f:
    json_ = json.load(f);

BULLSHARK_QUEST_1_START = 85
BULLSHARK_QUEST_1_END = 106
BULLSHARK_QUEST_2_START = 107
BULLSHARK_QUEST_2_END = 146
BULLSHARK_QUEST_3_START = 183
BULLSHARK_QUEST_3_END = 211
WINTER_QUEST_START = 250
WINTER_QUEST_END = 258

START_FROM_EPOCH = 20

FORMAT = 'png'


def plot_quests(ax, alpha=0.3, zorder=0, label=True):
    ax.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=alpha, color='red', label='Bullshark Quest 1' if label else None, zorder=0)
    ax.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=alpha, color='green', label='Bullshark Quest 2' if label else None, zorder=0)
    ax.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=alpha, color='blue', label='Bullshark Quest 3' if label else None, zorder=0)
    ax.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=alpha, color='cyan', label='Winter Quest' if label else None, zorder=0)


main_df = pd.DataFrame.from_dict(json_['epochs'], orient='index')
main_df.index = main_df.index.astype(int);
interval_df = pd.json_normalize(main_df['avg_interval_data'])

plt.rcParams.update({
    'font.size': 19,
    'text.usetex': True,
    'font.family': 'serif',
    'font.serif': ['Times']
})


# density -----------------------------------------------------------------------
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

plt.grid(which='minor', linewidth=0.5, linestyle=':', zorder=0)
plt.grid(which='major', linewidth=0.5, linestyle='-', zorder=0)

plot_quests(ax)
ax.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax.axhline(y=1, linestyle=':', linewidth=1, color='black')

ax.plot(main_df['density'][START_FROM_EPOCH:], linewidth=2, linestyle='-', marker='', color='black')

ax.set_ylabel('Density')
ax.set_xlabel('Epoch')
ax.minorticks_on()
ax.legend(fontsize=18)

fig.tight_layout()
plt.savefig(os.path.join(os.pardir, 'density.{}'.format(FORMAT)), format='{}'.format(FORMAT))
# density -----------------------------------------------------------------------


# transaction number ---------------------------------------------------------
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

plt.grid(which='minor', linewidth=0.5, linestyle=':', zorder=0)
plt.grid(which='major', linewidth=0.5, linestyle='-', zorder=0)

plot_quests(ax)

ax.axhline(y=0, linestyle=':', linewidth=1, color='black')

ax.plot(main_df['num_txs_total'][START_FROM_EPOCH:], linewidth=2, linestyle='-', marker='', color='black')

ax.set_ylabel('Number of transactions')
ax.set_xlabel('Epoch')
ax.minorticks_on()
ax.legend(fontsize=18)
ax.set_yscale('log')

fig.tight_layout()
plt.savefig(os.path.join(os.pardir, 'tx-number.{}'.format(FORMAT)), format='{}'.format(FORMAT))
# transaction number ---------------------------------------------------------


# contention degree ---------------------------------------------------------
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

plt.grid(which='minor', linewidth=0.5, linestyle=':', zorder=0)
plt.grid(which='major', linewidth=0.5, linestyle='-', zorder=0)

plot_quests(ax, label=False)

ax.axhline(y=1, linestyle=':', linewidth=1, color='black')

for col in interval_df:
    if 'degree' in col:
        ax.plot(interval_df[col][START_FROM_EPOCH:], linewidth=2, label='{} checkpoint{}'.format(col.split('.')[0],
            's' if not col.split('.')[0] == '1' else ''))
# ax.set_title('Average number of TXs touching the same shared object within an interval')

ax.set_xlabel('Epoch')
ax.set_ylabel('Contention degree')
# ax.set_yticks([0, 1, 2, 4, 6, 8, 10, 12, 14, 16, 18])
ax.minorticks_on()
ax.legend(fontsize=16)
ax.set_yscale('log')

fig.tight_layout()
plt.savefig(os.path.join(os.pardir, 'contention-degree.{}'.format(FORMAT)), format='{}'.format(FORMAT))
# contention degree ---------------------------------------------------------


# contended fraction ---------------------------------------------------------
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

plt.grid(which='minor', linewidth=0.5, linestyle=':', zorder=0)
plt.grid(which='major', linewidth=0.5, linestyle='-', zorder=0)

plot_quests(ax, label=False)

ax.axhline(y=0, linestyle=':', linewidth=1, color='black')

for col in interval_df:
    if not 'degree' in col:
        ax.plot(interval_df[col][START_FROM_EPOCH:], linewidth=2, label='{} checkpoint{}'.format(col.split('.')[0],
            's' if not col.split('.')[0] == '1' else ''))

ax.set_xlabel('Epoch')
ax.set_ylabel('Contended fraction')
ax.minorticks_on()
ax.legend(fontsize=16)

fig.tight_layout()
plt.savefig(os.path.join(os.pardir, 'contended-fraction.{}'.format(FORMAT)), format='{}'.format(FORMAT))
# contended fraction ---------------------------------------------------------


# object number per tx ---------------------------------------------------------
fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

plt.grid(which='minor', linewidth=0.5, linestyle=':', zorder=0)
plt.grid(which='major', linewidth=0.5, linestyle='-', zorder=0)

plot_quests(ax)

ax.axhline(y=1, linestyle=':', linewidth=1, color='black')

ax.plot(main_df['num_shared_objects_per_tx'][START_FROM_EPOCH:], linewidth=2, linestyle='-', marker='', color='black')

ax.set_ylabel('Number of shared objects')
ax.set_xlabel('Epoch')
ax.minorticks_on()
ax.legend(fontsize=18)

fig.tight_layout()
plt.savefig(os.path.join(os.pardir, 'obj-number.{}'.format(FORMAT)), format='{}'.format(FORMAT))
# object number per tx ---------------------------------------------------------
