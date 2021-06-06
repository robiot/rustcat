rustcat="./target/release/rc"
usrbin="/usr/bin/rc"


cargo build --release
if [ $? -eq 0 ]; then
    echo "Completed"
else
    echo "Failed to build. Dont run this as sudo"
    exit 1
fi

echo "Copying" $rustcat "To" $usrbin
sudo cp $rustcat $usrbin
if [ $? -eq 0 ]; then
    echo "Completed | run with 'rc'"
else
    echo "Failed"
    exit 1
fi