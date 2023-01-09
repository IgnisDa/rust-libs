#!/usr/bin/env sh

set -e
cd "$(dirname $0)"

remove_flag=""
if [ "$1" = "true" ]; then
    remove_flag="--remove-existing-container"
fi

# Generate certificate
mkdir -p tmp && cd tmp
rm -f temp-ssh-key*
ssh-keygen -q -N '' -t rsa -f temp-ssh-key
cd ../..

# Start container
devcontainer up $remove_flag \
    --mount "type=bind,source=$HOME/.config,target=/home/archlinux/.config" \
    --mount "type=bind,source=$(pwd)/.helix,target=/.helix" \
    --workspace-folder .

# Install vim (if needed) and add pub key to SSH allow list
devcontainer exec --workspace-folder . /.helix/init-helix.sh
devcontainer exec --workspace-folder . sudo /usr/local/share/ssh-init.sh

# Connect
ssh -t -i $PWD/.helix/tmp/temp-ssh-key \
    -o NoHostAuthenticationForLocalhost=yes \
    -o UserKnownHostsFile=/dev/null \
    -o GlobalKnownHostsFile=/dev/null \
    -p 2222 archlinux@localhost \
    helix /workspaces/rust-libs
