import json
import numpy as np
import matplotlib.pyplot as plt
from collections import OrderedDict
from pprint import pprint

FILE = './../results/workspace1/packages_data.json'
with open(FILE, 'r') as f:
    data = json.load(f);

N_APPS = 11
APP_ID_NAME_MAP = {
        '00b53b0f4174108627fbee72e2498b58d6a2714cded53fac537034c220d26302': 'Pyth Network',
        '0000000000000000000000000000000000000000000000000000000000000002': 'Sui Framework',
        '5306f64e312b581766351c07af79c72fcb1cd25147157fdc2f8ad76de9a3fb6a': 'Wormhole',
        'a0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66': 'Kriya DEX',
        'd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca': '0xd...1ca',
        '1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb': 'Cetus',
        '745a16ea148a7b3d1f6e68d0f16237f954e99197cd0ffb96e70c994c946d60d1': 'DeSuiLabs Coin Flip',
        'ceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f': 'ABEx Finance',
        '91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1': 'Turbos Finance',
        'efe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf': 'Scallop',
        '000000000000000000000000000000000000000000000000000000000000dee9': 'DeepBook',
        'Others': 'Others'
}

COLORS =  plt.cm.tab20((4./3*np.arange(20*3/4)).astype(int))

total_num_txs = 0
app_tx_num_map = {}
pkg_id_other_ids_map = {}
scanned_pkgs = set()

for pkg_id, pkg_data in data['packages'].items():
    total_num_txs += pkg_data['total_num_txs']
    if not pkg_id in scanned_pkgs:
        scanned_pkgs.add(pkg_id)
        app_tx_num_map[pkg_id] = pkg_data['total_num_txs']
        pkg_id_other_ids_map[pkg_id] = []
        for pkg_id2, pkg_data2 in data['packages'].items():
            if not pkg_id == pkg_id2:
                if pkg_data['types'].keys() == pkg_data2['types'].keys():
                    scanned_pkgs.add(pkg_id2)
                    app_tx_num_map[pkg_id] += pkg_data2['total_num_txs']
                    pkg_id_other_ids_map[pkg_id].append(pkg_id2)

print('Number of packages:    ', len(data['packages']))
print('Number of applications:', len(app_tx_num_map))

app_tx_num_map = OrderedDict(sorted(app_tx_num_map.items(), key=lambda x: x[1], reverse=True))

i = 0
app_tx_num_map2 = OrderedDict()

for id_, tx_num in app_tx_num_map.items():
    if i < N_APPS:
        app_tx_num_map2[id_] = tx_num
        i += 1
    else:
        break
app_tx_num_map2['Others'] = total_num_txs - sum(app_tx_num_map2.values())
# pprint(app_tx_num_map2)

plt.rcParams.update({
    'font.size': 16,
    'text.usetex': True,
    'font.family': 'serif',
    'font.serif': ['Times']
})

fig, (ax, ax2) = plt.subplots(nrows=1, ncols=2, figsize=(9, 6), width_ratios=[3, 1], dpi=300)

labels = []
for k in app_tx_num_map2.keys():
    try:
        labels.append(APP_ID_NAME_MAP[k])
    except KeyError:
        pass

patches, texts, autotexts = ax.pie(app_tx_num_map2.values(), labels=None, autopct='%1.1f\%%', startangle=0, pctdistance=0.8, colors=COLORS)

bbox_props = dict(boxstyle="square,pad=0.3", fc="w", ec="k", lw=0.72)
kw = dict(arrowprops=dict(arrowstyle="-"),
          bbox=bbox_props, zorder=0, va="center")

k = 0
for i, patch in enumerate(patches):
    if float(autotexts[i].get_text()[:-3]) < 5.0:
        ang = (patch.theta2 - patch.theta1) / 2.0 + patch.theta1
        x = np.cos(np.deg2rad(ang))
        y = np.sin(np.deg2rad(ang))
        horizontalalignment = {-1: "right", 1: "left"}[int(np.sign(x))]
        connectionstyle = f"angle,angleA=0,angleB={ang}"
        kw["arrowprops"].update({"connectionstyle": connectionstyle})
        ax.annotate(autotexts[i].get_text(), xy=(x, y), xytext=(0.6*np.sign(x) + 0.3*k, (1.25 - 0.01*k)*y),
                    horizontalalignment=horizontalalignment, **kw)
        autotexts[i].set_visible(False)
        k += 1

ax2.set_frame_on(False)
ax2.get_xaxis().set_visible(False)
ax2.get_yaxis().set_visible(False)
fig.legend(title='Applications:', labels=labels)

fig.tight_layout()
ax.set_position([-0.18, 0.04, 1.0, 1.0])
plt.savefig('./../results/workspace1/app_pie.pdf', format='pdf')
