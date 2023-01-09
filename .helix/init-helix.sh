#!/bin/sh
set -eux

# Copy generated keys
mkdir -p $HOME/.ssh
cat /.helix/tmp/temp-ssh-key.pub > $HOME/.ssh/authorized_keys
chmod 644 $HOME/.ssh/authorized_keys
chmod 700 $HOME/.ssh
