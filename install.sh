rustcat="./target/debug/rc"
usrbin="/usr/bin/rc"

echo "Copying" $rustcat "To" $usrbin
cp $rustcat $usrbin
if [ $? -eq 0 ]; then
    echo "Completed"
else
    echo "Failed, Remember to run with sudo"
    exit 1
fi