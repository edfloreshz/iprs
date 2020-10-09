rustup target add x86_64-apple-darwin
git clone https://github.com/tpoechtrager/osxcross
cd osxcross
wget -nc https://s3.dockerproject.org/darwin/v2/MacOSX10.10.sdk.tar.xz
mv MacOSX10.10.sdk.tar.xz tarballs/
UNATTENDED=yes OSX_VERSION_MIN=10.7 ./build.sh
sudo cp -r target /usr/local/bin/osxcross/
echo "export PATH=/usr/local/bin/osxcross/target/bin:$PATH" >> ~/.zshrc
