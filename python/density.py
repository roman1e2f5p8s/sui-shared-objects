import os
import json
import pandas as pd
import matplotlib.pyplot as plt

FILE = './../results/workspace1/epoch_density_data.json'
with open(FILE, 'r') as f:
    json_ = json.load(f);

main_df = pd.DataFrame.from_dict(json_['epochs'], orient='index')
main_df.index = main_df.index.astype(int);
interval_df = pd.json_normalize(main_df['avg_interval_data'])

plt.rcParams.update({
    'font.size': 18,
    'text.usetex': True,
    'font.family': 'serif',
    'font.serif': ['Times']
})

fig, ax = plt.subplots(nrows=1, ncols=1, figsize=(9, 6), dpi=300)

ax.axhline(y=0, linestyle=':', linewidth=1, color='black')
#ax.axvline(x=85, linestyle='-.', linewidth=1, color='red', label='Bullshark Quest 1 start')
ax.plot(main_df['density'], linewidth=2, linestyle='--', marker='o', color='green')
#ax.set_title('Percentage of TXs involving shared objects on the Sui mainnet per epoch')
ax.set_ylabel('Density')
ax.set_xlabel('Epoch')
ax.minorticks_on()
#ax.legend()

fig.tight_layout()
plt.savefig('./../results/workspace1/density.pdf', format='pdf')
