rustcat="./target/debug/rc"
usrbin="/usr/bin/rc"


echo "Making" $rustcat "into a executable"
chmod +x $rustcat
echo "Completed"

echo "Copying" $rustcat "To" $usrbin
cp $rustcat $usrbin
echo "Completed"