FROM subdarkdex/generic-chain:v0.1.0 as generic

FROM debian:buster-slim as collator
RUN apt-get update && apt-get install jq curl bash -y && \
    curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /wait-for-it.sh && \
    curl -sL https://deb.nodesource.com/setup_12.x | bash - 
COPY --from=generic \
    /generic/target/release/generic-parachain-collator /usr/bin
COPY ./start_generic_collator.sh /usr/bin


FROM debian:buster-slim as runtime
COPY --from=generic \
    /generic_chain/target/release/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
    /var/opt/
RUN mkdir /runtime
RUN cp -v /var/opt/parachain_runtime.compact.wasm /runtime
