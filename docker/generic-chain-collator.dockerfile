FROM subdarkdex/generic-chain:v0.2.0 as generic

FROM debian:stretch-slim as collator

LABEL link.subdex.image.authors="subdex" \
	link.subdex.image.title="generic_chain" \
	link.subdex.image.description="Generic parachain with assets to demo token dealer" 


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

		curl -sL https://deb.nodesource.com/setup_12.x | bash - \

# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
# add user
	useradd -m -u 1000 -U -s /bin/sh -d /generic generic 

COPY --from=generic \
    /generic_chain/target/release/parachain-collator /usr/local/bin
COPY ./start_generic_collator.sh /usr/local/bin

USER generic

FROM debian:buster-slim as runtime
COPY --from=generic \
    /generic_chain/target/release/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
    /var/opt/
RUN mkdir /runtime
RUN cp -v /var/opt/parachain_runtime.compact.wasm /runtime
