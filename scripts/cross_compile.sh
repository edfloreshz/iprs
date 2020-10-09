IPSSVER="v0.1.1"

cargo build --release
cargo build --target=x86_64-apple-darwin --release
cargo build --target=x86_64-pc-windows-gnu --release

mkdir -p release
mkdir -p release/ipss.$IPSSVER-x86_64-linux-gnu-alpha/
mkdir -p release/ipss.$IPSSVER-x86_64-apple-darwin-alpha
mkdir -p release/ipss.$IPSSVER-x86_64-windows-gnu-alpha
cp ../target/release/ipss release/ipss.$IPSSVER-x86_64-linux-gnu-alpha/
cp ../target/x86_64-apple-darwin/release/ipss release/ipss.$IPSSVER-x86_64-apple-darwin-alpha
cp ../target/x86_64-pc-windows-gnu/release/ipss.exe release/ipss.$IPSSVER-x86_64-windows-gnu-alpha

echo "#\!/bin/bash
if sudo cp ipss /usr/bin/; then
    echo \"IPSS is now installed, try running it with 'ipss'.\"
    echo \" \"
    echo \"If you are unable to run it, try opening a new terminal and try again.\"
else
    echo \"Installation failed.\"
fi" > release/ipss.$IPSSVER-x86_64-linux-gnu-alpha/install.sh
echo "#\!/bin/sh
if sudo cp \$( cd \"\$( dirname \"\$0\" )\" && pwd )/ipss /usr/local/bin/; then
    sudo chmod 755 /usr/local/bin/ipss
    sudo xattr -d com.apple.quarantine /usr/local/bin/ipss
    echo \"IPSS is now installed, try running it with 'ipss'.\"
    echo \" \"
    echo \"If you are unable to run it, try opening a new terminal and try again.\"
else
    echo \"Installation failed.\"
fi" > release/ipss.$IPSSVER-x86_64-apple-darwin-alpha/install.command
echo "echo on
xcopy ipss.exe  $HOME\.ipss
echo \" \"
echo \"IPSS is now installed, add $HOME\.ipss\ to your PATH variable to run.\"
pause" > release/ipss.$IPSSVER-x86_64-windows-gnu-alpha/install.ps1

echo "#\!/bin/bash
if sudo rm -rf /usr/bin/ipss; then
    echo \"IPSS is now uninstalled.\"
else
    echo \"Something went wrong.\"
fi" > release/ipss.$IPSSVER-x86_64-linux-gnu-alpha/uninstall.sh
echo "#\!/bin/bash
if sudo rm -rf /usr/local/bin/ipss; then
    echo \"IPSS is now uninstalled.\"
else
    echo \"Something went wrong.\"
fi" > release/ipss.$IPSSVER-x86_64-apple-darwin-alpha/uninstall.command
echo "Remove-Item Path $HOME\.ipss\ipss.exe -Recurse
echo \"IPSS is now uninstalled.\"
pause" > release/ipss.$IPSSVER-x86_64-windows-gnu-alpha/uninstall.ps1

zip -r release/ipss.$IPSSVER-x86_64-linux-gnu-alpha.zip release/ipss.$IPSSVER-x86_64-linux-gnu-alpha
zip -r release/ipss.$IPSSVER-x86_64-apple-darwin-alpha.zip release/ipss.$IPSSVER-x86_64-apple-darwin-alpha
zip -r release/ipss.$IPSSVER-x86_64-windows-gnu-alpha.zip release/ipss.$IPSSVER-x86_64-windows-gnu-alpha