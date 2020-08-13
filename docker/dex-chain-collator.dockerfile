FROM belsyuen/dex-chain:v0.1.0 as dex

# the collator stage is normally built once, cached, and then ignored, but can
# be specified with the --target build flag. This adds some extra tooling to the
# image, which is required for a launcher script. The script simply adds two
# arguments to the list passed in:
#
#   --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/PEER_ID
#
# with the appropriate ip and ID for both Alice and Bob
FROM debian:buster-slim as collator
RUN apt-get update && apt-get install jq curl bash -y && \
    curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /wait-for-it.sh && \
    curl -sL https://deb.nodesource.com/setup_12.x | bash - && \
    apt-get install -y nodejs && \
    npm install --global yarn && \
    yarn global add @polkadot/api-cli@0.18.1
COPY --from=dex \
    /dex_chain/target/release/dex-chain /usr/bin
COPY ./start_dex_collator.sh /usr/bin
# This queries bootnodes and run collator, a binary copied

# the runtime stage is normally built once, cached, and ignored, but can be
# specified with the --target build flag. This just preserves one of the dex's
# outputs, which can then be moved into a volume at runtime
FROM debian:buster-slim as runtime
COPY --from=dex \
    /dex_chain/target/release/wbuild/dex-chain-runtime/dex_chain_runtime.compact.wasm \
    /var/opt/
RUN mkdir /runtime
RUN cp -v /var/opt/dex_chain_runtime.compact.wasm /runtime


# default
FROM debian:buster-slim
COPY --from=dex \
    /dex_chain/target/release/dex-chain /usr/bin
CMD ["/usr/bin/dex-chain"]
