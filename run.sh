echo "------------------------------------ "
echo "1/3: Running ./target/release/density ..."
./target/release/density
echo

echo "------------------------------------ "
echo "2/3: Plotting ..."
cd python
. .pyenv/bin/activate
python main.py
deactivate
cd ..
echo

echo "------------------------------------ "
echo "3/3: Running ./target/release/query_obj ..."
./target/release/query_obj
echo " ----------------------------------- "
