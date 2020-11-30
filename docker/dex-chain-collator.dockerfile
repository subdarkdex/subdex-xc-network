FROM subdarkdex/subdex-chain:v0.2.0 as dex

FROM debian:stretch-slim as collator

LABEL link.subdex.image.authors="subdex" \
	link.subdex.image.title="subdex_chain" \
	link.subdex.image.description="DEX on substrate for Cross-Chain asset transfer" \

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		jq \
		libssl1.1 \
		ca-certificates \
		curl && \
		curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
		chmod +x /wait-for-it.sh && \
		curl -sL https://deb.nodesource.com/setup_12.x | bash - 
# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
	useradd -m -u 1000 -U -s /bin/sh -d /subdex subdex

COPY --from=dex \
    /subdex_chain/target/release/parachain-collator /usr/local/bin
COPY ./start_dex_collator.sh /usr/local/bin

USER subdex
# This queries bootnodes and run collator, a binary copied

# the runtime stage is normally built once, cached, and ignored, but can be
# specified with the --target build flag. This just preserves one of the dex's
# outputs, which can then be moved into a volume at runtime
FROM debian:buster-slim as runtime
COPY --from=dex \
    /subdex_chain/target/release/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
    /var/opt/
RUN mkdir /runtime
RUN cp -v /var/opt/parachain_runtime.compact.wasm /runtime

