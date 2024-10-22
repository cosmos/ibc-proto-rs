#!/usr/bin/env bash

set -eou pipefail

# syn-protobuf.sh is a bash script to sync the protobuf
# files using ibc-proto-compiler. This script will checkout
# the protobuf files from the git versions specified in
# proto/src/prost/IBC_GO_COMMIT. If you want to sync
# the protobuf files to a newer version, modify the
# corresponding of those 2 files by specifying the commit ID
# that you wish to checkout from.

# This script should be run from the root directory of ibc-proto-rs.

# We can specify where to clone the git repositories
# for cosmos-sdk and ibc-go. By default they are cloned
# to /tmp/cosmos-sdk.git and /tmp/ibc-go.git.
# We can override this to existing directories
# that already have a clone of the repositories,
# so that there is no need to clone the entire
# repositories over and over again every time
# the script is called.

CACHE_PATH="${XDG_CACHE_HOME:-$HOME/.cache}"/ibc-proto-rs-build
IBC_GO_GIT="${IBC_GO_GIT:-$CACHE_PATH/ibc-go.git}"
COSMOS_ICS_GIT="${COSMOS_ICS_GIT:-$CACHE_PATH/interchain-security.git}"
NFT_TRANSFER_GIT="${NFT_TRANSFER_GIT:-$CACHE_PATH/nft-transfer.git}"

IBC_GO_COMMIT="$(cat src/IBC_GO_COMMIT)"
INTERCHAIN_SECURITY_COMMIT="$(cat src/INTERCHAIN_SECURITY_COMMIT)"
NFT_TRANSFER_COMMIT="$(cat src/NFT_TRANSFER_COMMIT)"

echo "IBC_GO_COMMIT: $IBC_GO_COMMIT"
echo "INTERCHAIN_SECURITY_COMMIT: $INTERCHAIN_SECURITY_COMMIT"
echo "NFT_TRANSFER_COMMIT: $NFT_TRANSFER_COMMIT"

# Use either --ics-commit flag for commit ID,
# or --ics-tag for git tag. Because we can't modify
# proto-compiler to have smart detection on that.

if [[ "$INTERCHAIN_SECURITY_COMMIT" =~ ^[a-zA-Z0-9]{40}$ ]]
then
    ICS_COMMIT_OPTION="--ics-commit"
else
    ICS_COMMIT_OPTION="--ics-tag"
fi

# If the git directories does not exist, clone them as
# bare git repositories so that no local modification
# can be done there.

if [[ ! -e "$COSMOS_ICS_GIT" ]]
then
    echo "Cloning interchain-security source code to as bare git repository to $COSMOS_ICS_GIT"
    git clone --mirror https://github.com/cosmos/interchain-security.git "$COSMOS_ICS_GIT"
else
    echo "Using existing interchain-security bare git repository at $COSMOS_ICS_GIT"
fi

# If the git directories does not exist, clone them as
# bare git repositories so that no local modification
# can be done there.

if [[ ! -e "$IBC_GO_GIT" ]]
then
    echo "Cloning ibc-go source code to as bare git repository to $IBC_GO_GIT"
    git clone --mirror https://github.com/cosmos/ibc-go.git "$IBC_GO_GIT"
else
    echo "Using existing ibc-go bare git repository at $IBC_GO_GIT"
fi

if [[ ! -e "$NFT_TRANSFER_GIT" ]]
then
    echo "Cloning nft-transfer source code to as bare git repository to $NFT_TRANSFER_GIT"
    git clone --mirror https://github.com/bianjieai/nft-transfer.git "$NFT_TRANSFER_GIT"
else
    echo "Using existing nft-transfer bare git repository at $NFT_TRANSFER_GIT"
fi


# Update the repositories using git fetch. This is so that
# we keep local copies of the repositories up to sync first.
pushd "$COSMOS_ICS_GIT"
git fetch
popd

pushd "$IBC_GO_GIT"
git fetch
popd

pushd "$NFT_TRANSFER_GIT"
git fetch
popd

# Create a new temporary directory to check out the
# actual source files from the bare git repositories.
# This is so that we do not accidentally use an unclean
# local copy of the source files to generate the protobuf.
COSMOS_ICS_DIR=$(mktemp -d /tmp/interchain-security-XXXXXXXX)

pushd "$COSMOS_ICS_DIR"
git clone "$COSMOS_ICS_GIT" .
git checkout "$INTERCHAIN_SECURITY_COMMIT"

cd proto
buf mod prune
buf mod update
buf export -v -o ../proto-include
popd

IBC_GO_DIR=$(mktemp -d /tmp/ibc-go-XXXXXXXX)

pushd "$IBC_GO_DIR"
git clone "$IBC_GO_GIT" .
git checkout "$IBC_GO_COMMIT"

cd proto
buf export -v -o ../proto-include
popd

NFT_TRANSFER_DIR=$(mktemp -d /tmp/nft-transfer-XXXXXXXX)

pushd "$NFT_TRANSFER_DIR"
git clone "$NFT_TRANSFER_GIT" .
git checkout "$NFT_TRANSFER_COMMIT"

cd proto
buf export -v -o ../proto-include
rm ../proto-include/ibc/core/client/v1/client.proto
popd

# Remove the existing generated protobuf files
# so that the newly generated code does not
# contain removed files.

PROST_DIR="prost"

rm -rf "src/$PROST_DIR"
mkdir -p "src/$PROST_DIR"

cd tools/proto-compiler

cargo build

# Run the proto-compiler twice,
# once with transport and once without


cargo run -- compile \
  --transport \
  --ics "$COSMOS_ICS_DIR/proto-include" \
  --ibc "$IBC_GO_DIR/proto-include" \
  --nft "$NFT_TRANSFER_DIR/proto-include" \
  --out "../../src/$PROST_DIR"

cd ../..

# Remove generated ICS23 code because it is not used,
# we instead re-exports the `ics23` crate type definitions.
rm -f "src/$PROST_DIR/cosmos.ics23.v1.rs"

# Remove leftover Cosmos SDK modules.
rm -f "src/$PROST_DIR/cosmos.base.store.v1beta1.rs"
rm -f "src/$PROST_DIR/cosmos.auth.v1beta1.rs"
rm -f "src/$PROST_DIR/cosmos.base.query.v1beta1.rs"
rm -f "src/$PROST_DIR/cosmos.base.v1beta1.rs"
rm -f "src/$PROST_DIR/cosmos.staking.v1beta1.rs"
rm -f "src/$PROST_DIR/cosmos.upgrade.v1beta1.rs"
rm -f "src/$PROST_DIR/cosmos_proto.rs"

# The Tendermint ABCI protos are unused from within ibc-proto
rm -f "src/$PROST_DIR/tendermint.abci.rs"

# Remove leftover Google HTTP configuration protos.
rm -f "src/$PROST_DIR/google.api.rs"

# Remove the temporary checkouts of the repositories
rm -rf "$COSMOS_ICS_DIR"
rm -rf "$IBC_GO_DIR"
rm -rf "$NFT_TRANSFER_DIR"
