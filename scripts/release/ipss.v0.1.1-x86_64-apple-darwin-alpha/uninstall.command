#\!/bin/bash
if sudo rm -rf /usr/local/bin/ipss; then
    echo "IPSS is now uninstalled."
else
    echo "Something went wrong."
fi
