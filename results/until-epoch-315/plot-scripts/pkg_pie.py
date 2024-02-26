import os
import json
import numpy as np
import matplotlib.pyplot as plt
from collections import OrderedDict
from pprint import pprint

FILE = os.path.join(os.pardir, 'packages_data.json')
with open(FILE, 'r') as f:
    data = json.load(f);

N_PKGS = 12
PKG_ID_NAME_MAP = {
        '8d97f1cd6ac663735be08d1d2b6d02a159e711586461306ce60a2b7a6a565a9e': 'Pyth Network 2',
        '0000000000000000000000000000000000000000000000000000000000000002': 'Sui Framework',
        '5306f64e312b581766351c07af79c72fcb1cd25147157fdc2f8ad76de9a3fb6a': 'Wormhole',
        '00b53b0f4174108627fbee72e2498b58d6a2714cded53fac537034c220d26302': 'Pyth Network 1',
        '000000000000000000000000000000000000000000000000000000000000dee9': 'DeepBook',
        '1eabed72c53feb3805120a081dc15963c204dc8d091542592abaf7a35689b2fb': 'Cetus 4',
        'a0eba10b173538c8fecca1dff298e488402cc9ff374f8a12ca7758eebe830b66': 'Kriya DEX',
        'd899cf7d2b5db716bd2cf55599fb0d5ee38a3061e7b6bb6eebf73fa5bc4c81ca': '0xd...1ca',
        '830fe26674dc638af7c3d84030e2575f44a2bdc1baa1f4757cfe010a4b106b6a': 'Movescriptions 1',
        'cb4e1ee2a3d6323c70e7b06a8638de6736982cbdc08317d33e6f098747e2b438': 'Bluefin 3',
        '91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1': 'Turbos Finance 1',
        '51179c2df7466220b513901c52412258942a1e041fccb973e92a53c29e1a09ed': 'Reference Price Oracle',

        '745a16ea148a7b3d1f6e68d0f16237f954e99197cd0ffb96e70c994c946d60d1': 'DeSuiLabs Coin Flip 2',
        'ceab84acf6bf70f503c3b0627acaff6b3f84cee0f2d7ed53d00fa6c2a168d14f': 'ABEx Finance',
        'efe8b36d5b2e43728cc323298626b83177803521d195cfb11e15b910e892fddf': 'Scallop',

        'Others': 'Others'
}

COLORS =  plt.cm.tab20((4./3*np.arange(20*3/4)).astype(int))

total_num_txs = 0
pkg_tx_num_map = OrderedDict()
i = 0

for pkg_id, pkg_data in data['packages'].items():
    total_num_txs += pkg_data['total_num_txs']
    if i < N_PKGS:
        pkg_tx_num_map[pkg_id] = pkg_data['total_num_txs']
        i += 1
pkg_tx_num_map['Others'] = total_num_txs - sum(pkg_tx_num_map.values())
# pprint(pkg_tx_num_map)

plt.rcParams.update({
    'font.size': 18,
    'text.usetex': True,
    'font.family': 'serif',
    'font.serif': ['Times']
})

fig, (ax, ax2) = plt.subplots(nrows=1, ncols=2, figsize=(9, 6), width_ratios=[3, 1], dpi=300)

labels = []
for k in pkg_tx_num_map.keys():
    try:
        labels.append(PKG_ID_NAME_MAP[k])
    except KeyError:
        pass

patches, texts, autotexts = ax.pie(pkg_tx_num_map.values(), labels=None, autopct='%1.1f\%%', startangle=-30, pctdistance=0.8, colors=COLORS)

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
        if k < 3:
            ax.annotate(autotexts[i].get_text(), xy=(x, y), xytext=(-0.3*np.sign(x) - 0.3*k, (1.09 + 0.12*k)*y),
                    horizontalalignment=horizontalalignment, **kw)
        else:
            ax.annotate(autotexts[i].get_text(), xy=(x, y), xytext=(0.1*np.sign(x) + 0.3*k, (1.60 - 0.08*k)*y),
                    horizontalalignment=horizontalalignment, **kw)
        autotexts[i].set_visible(False)
        k += 1

ax2.set_frame_on(False)
ax2.get_xaxis().set_visible(False)
ax2.get_yaxis().set_visible(False)
fig.legend(title='Packages:', labels=labels, fontsize=16)

fig.tight_layout()
ax.set_position([-0.18, 0.05, 1.0, 1.0])
plt.savefig(os.path.join(os.pardir, 'pkg_pie.pdf'), format='pdf')
