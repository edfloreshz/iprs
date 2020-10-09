#\!/bin/bash
if sudo cp ipss /usr/bin/; then
    echo "IPSS is now installed, try running it with 'ipss'."
    echo " "
    echo "If you are unable to run it, try opening a new terminal and try again."
else
    echo "Installation failed."
fi
