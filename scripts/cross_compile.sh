IPRSVER="v0.1.1"

cargo build --release
cargo build --target=x86_64-apple-darwin --release
cargo build --target=x86_64-pc-windows-gnu --release

mkdir -p release
mkdir -p release/iprs.$IPRSVER-x86_64-linux-gnu-alpha/
mkdir -p release/iprs.$IPRSVER-x86_64-apple-darwin-alpha
mkdir -p release/iprs.$IPRSVER-x86_64-windows-gnu-alpha
cp ../target/release/iprs release/iprs.$IPRSVER-x86_64-linux-gnu-alpha/
cp ../target/x86_64-apple-darwin/release/iprs release/iprs.$IPRSVER-x86_64-apple-darwin-alpha
cp ../target/x86_64-pc-windows-gnu/release/iprs.exe release/iprs.$IPRSVER-x86_64-windows-gnu-alpha

echo "#\!/bin/bash
if sudo cp iprs /usr/bin/; then
    echo \"IPRS is now installed, try running it with 'iprs'.\"
    echo \" \"
    echo \"If you are unable to run it, try opening a new terminal and try again.\"
else
    echo \"Installation failed.\"
fi" > release/iprs.$IPRSVER-x86_64-linux-gnu-alpha/install.sh
echo "#\!/bin/sh
if sudo cp \$( cd \"\$( dirname \"\$0\" )\" && pwd )/iprs /usr/local/bin/; then
    sudo chmod 755 /usr/local/bin/iprs
    sudo xattr -d com.apple.quarantine /usr/local/bin/iprs
    echo \"IPRS is now installed, try running it with 'iprs'.\"
    echo \" \"
    echo \"If you are unable to run it, try opening a new terminal and try again.\"
else
    echo \"Installation failed.\"
fi" > release/iprs.$IPRSVER-x86_64-apple-darwin-alpha/install.command
echo "echo on
xcopy iprs.exe  \$HOME\.iprs
echo \" \"
echo \"IPRS is now installed, add $HOME\.iprs\ to your PATH variable to run.\"
pause" > release/iprs.$IPRSVER-x86_64-windows-gnu-alpha/install.ps1

echo "#\!/bin/bash
if sudo rm -rf /usr/bin/iprs; then
    echo \"IPRS is now uninstalled.\"
else
    echo \"Something went wrong.\"
fi" > release/iprs.$IPRSVER-x86_64-linux-gnu-alpha/uninstall.sh
echo "#\!/bin/bash
if sudo rm -rf /usr/local/bin/iprs; then
    echo \"IPRS is now uninstalled.\"
else
    echo \"Something went wrong.\"
fi" > release/iprs.$IPRSVER-x86_64-apple-darwin-alpha/uninstall.command
echo "Remove-Item -Path \$HOME\.iprs\iprs.exe -Recurse
echo \"IPRS is now uninstalled.\"
pause" > release/iprs.$IPRSVER-x86_64-windows-gnu-alpha/uninstall.ps1

chmod -R 755 release/

zip -r release/iprs.$IPRSVER-x86_64-linux-gnu-alpha.zip release/iprs.$IPRSVER-x86_64-linux-gnu-alpha
zip -r release/iprs.$IPRSVER-x86_64-apple-darwin-alpha.zip release/iprs.$IPRSVER-x86_64-apple-darwin-alpha
zip -r release/iprs.$IPRSVER-x86_64-windows-gnu-alpha.zip release/iprs.$IPRSVER-x86_64-windows-gnu-alpha