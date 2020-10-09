#\!/bin/sh
if sudo cp $( cd "$( dirname "$0" )" && pwd )/ipss /usr/local/bin/; then
    sudo chmod 755 /usr/local/bin/ipss
    sudo xattr -d com.apple.quarantine /usr/local/bin/ipss
    echo "IPSS is now installed, try running it with 'ipss'."
    echo " "
    echo "If you are unable to run it, try opening a new terminal and try again."
else
    echo "Installation failed."
fi
