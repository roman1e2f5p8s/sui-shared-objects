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
WINTER_QUEST_END = 258

MARKER_EVERY = 5


def plot_quests(ax, alpha=0.3, zorder=0, label=True):
    ax.axvspan(BULLSHARK_QUEST_1_START, BULLSHARK_QUEST_1_END, alpha=alpha, color='red', label='Bullshark Quest 1' if label else None, zorder=0)
    ax.axvspan(BULLSHARK_QUEST_2_START, BULLSHARK_QUEST_2_END, alpha=alpha, color='green', label='Bullshark Quest 2' if label else None, zorder=0)
    ax.axvspan(BULLSHARK_QUEST_3_START, BULLSHARK_QUEST_3_END, alpha=alpha, color='blue', label='Bullshark Quest 3' if label else None, zorder=0)
    ax.axvspan(WINTER_QUEST_START, WINTER_QUEST_END, alpha=alpha, color='cyan', label='Winter Quest' if label else None, zorder=0)


def plot_fig(
        ax,
        y,
        start_from=0,
        quests=False,
        linewidth=2,
        linestyle='-',
        color='black',
        marker='',
        markersize=0,
        markevery=1,
        label='',
        alpha=0.3,
        add_y0_line=False,
        add_y1_line=False,
        title='',
        xlabel='',
        ylabel='',
        minorticks=False,
        logscale=False,
        legend=False,
        ):
    if add_y0_line:
        ax.axhline(y=0, linestyle=':', linewidth=1, color='black', zorder=1)

    if add_y1_line:
        ax.axhline(y=1, linestyle=':', linewidth=1, color='black', zorder=1)

    ax.plot(
            y[start_from:], 
            linewidth=linewidth,
            linestyle=linestyle,
            color=color,
            marker=marker,
            markersize=markersize,
            markevery=markevery,
            label=label if label else None,
            zorder=2,
    )

    if quests:
        plot_quests(ax, alpha=alpha, zorder=0)

    if title:
        ax.set_title(title)
    
    if xlabel:
        ax.set_xlabel(xlabel)

    if ylabel:
        ax.set_ylabel(ylabel)

    if logscale:
        ax.set_yscale('log')

    if minorticks:
        ax.minorticks_on()

    if legend:
        ax.legend()


main_df = pd.DataFrame.from_dict(json_['epochs'], orient='index')
main_df.index = main_df.index.astype(int);
interval_df = pd.json_normalize(main_df['avg_interval_data'])
print('Total number of scanned TXs: {}'.format(main_df['num_txs_total'].sum()))

plt.rcParams.update({'font.size': 14, 'font.family': 'sans-serif'})

fig, (ax1, ax2, ax3, ax4, ax5, ax6, ax7) = plt.subplots(nrows=NUM_SUBPLOTS, ncols=1, figsize=(10, NUM_SUBPLOTS * 7))


# Plot the total number of TXs and number of TXs touching shared objects -----
plot_fig(
        ax=ax1,
        y=main_df['num_txs_total'],
        start_from=20,
        label='Total',
        ylabel='TX number',
        xlabel='Epoch',
        minorticks=True,
        logscale=True,
)
plot_fig(
        ax=ax1,
        y=main_df['num_txs_touching_shared_objs'],
        start_from=20,
        quests=True,
        alpha=0.3,
        linestyle='--',
        color='blue',
        label='Touch shared obj.',
        legend=True,
)
# ------------------------------------------------------------------------------


# Plot the number of TXs touching shared objects and number of TXs touching at 
# least one shared object by a mutable reference
#plot_fig(
#        ax=ax2,
#        y=main_df['num_txs_touching_shared_objs'],
#        start_from=20,
#        label='Touch shared obj.',
#        ylabel='TX number',
#        minorticks=True,
#        logscale=True,
#)
#plot_fig(
#        ax=ax2,
#        y=main_df['num_txs_touching_at_least_one_shared_obj_by_mut'],
#        start_from=20,
#        quests=True,
#        alpha=0.3,
#        linestyle='',
#        marker='+',
#        markersize='7',
#        markevery=2,
#        color='red',
#        label='Touch >=1 by &mut',
#        legend=True,
#)
# ------------------------------------------------------------------------------


# Plot the ratio of the number of TXs touching at least one shared object by a
# mutable reference to the number of shared-object transactions
plot_fig(
        ax=ax2,
        y=main_df['num_txs_touching_at_least_one_shared_obj_by_mut'] / main_df['num_txs_touching_shared_objs'],
        start_from=20,
        quests=True,
        add_y1_line=True,
        xlabel='Epoch',
        title='Ratio of TX num. touching >=1 shared obj. by &mut to shared-object TX num.',
        minorticks=True,
        logscale=False,
        legend=True,
)
# ------------------------------------------------------------------------------


# Plot the density of shared-object TXs and density of transactions touching
# at least one shared object by a mutable reference
plot_fig(
        ax=ax3,
        y=main_df['density'],
        start_from=20,
        xlabel='Epoch',
        label='All shared obj.',
        ylabel='Density',
        add_y1_line=True,
        minorticks=True,
)
plot_fig(
        ax=ax3,
        y=main_df['density_mut'],
        start_from=20,
        quests=True,
        alpha=0.3,
        linestyle='',
        marker='+',
        markersize=7,
        markevery=2,
        color='red',
        label='>=1 by &mut',
        legend=True,
)
# ------------------------------------------------------------------------------


# Plot of the number of shared object per epoch
plot_fig(
        ax=ax4,
        y=main_df['num_shared_objects_total'],
        start_from=20,
        ylabel='Shared object number',
        xlabel='Epoch',
        label='Total',
        minorticks=True,
        logscale=True,
)
plot_fig(
        ax=ax4,
        y=main_df['num_shared_objects_per_epoch'],
        start_from=20,
        quests=True,
        linestyle='--',
        color='blue',
        label='Per epoch',
        legend=True,
)
# ------------------------------------------------------------------------------


# Plot the average contention degree
ax5.axhline(y=0, linestyle=':', linewidth=1, color='black')
ax5.axhline(y=1, linestyle='-.', linewidth=1, color='black')
for col in interval_df:
    if 'degree' in col:
        ax5.plot(interval_df[col][20:], linewidth=2, label='Interval: {} checkpoints'.format(col.split('.')[0]))
ax5.set_title('Ratio of shared-obj. TX num. to shared obj. num. touched within interval')
ax5.set_ylabel('Avg contention degree')
ax5.minorticks_on()
ax5.set_yscale('log')
plot_quests(ax=ax5, label=False)
ax5.legend()
# ------------------------------------------------------------------------------


# Plot the average contended fraction
ax6.axhline(y=0, linestyle=':', linewidth=1, color='black')
for col in interval_df:
    if not 'degree' in col:
        ax6.plot(interval_df[col][20:], linewidth=2, label='Interval: {} checkpoints'.format(col.split('.')[0]))
ax6.set_title('Ratio of shared obj. num. touched by >1 TX to shared obj. num. within interval')
ax6.set_xlabel('Epoch')
ax6.set_ylabel('Avg contended fraction')
ax6.minorticks_on()
# ax6.set_yscale('log')
plot_quests(ax=ax6, label=False)
ax6.legend()
# ------------------------------------------------------------------------------


# Plot the average number of shared objects per TX
plot_fig(
        ax=ax7,
        y=main_df['num_shared_objects_per_tx'],
        start_from=20,
        quests=True,
        add_y1_line=True,
        xlabel='Epoch',
        ylabel='Avg shared obj. number',
        title='Number of shared objects touched by one TX on average',
        minorticks=True,
        logscale=False,
        legend=True,
)
# ------------------------------------------------------------------------------


fig.tight_layout()
plt.savefig('./../results/workspace1/figure.png', format='png')
