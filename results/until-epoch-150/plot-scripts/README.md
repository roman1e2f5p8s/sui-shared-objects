Set up a virtual environment and install dependencies:
- Create a virtual environment: `python -m venv .pyenv`.
- Activate virtual environment: `. .pyenv/bin/activate`.
- Install requirements: `pip install -r requirements.txt`.

To plot figures, run:
- `python main.py`: to plot all metrics on one (long) figure.
- `python metrics.py`: to plot metrics each on a separate figure/file.
- `python app_pie.py`: to plot a pie chart of the most used dApps (involving shared objects) on Sui.
- `python pkg_pie.py`: to plot a pie chart of the most used packages (involving shared objects) on Sui.
- `python obj_pie.py`: to plot a pie chart of the most used shared objects on Sui.
