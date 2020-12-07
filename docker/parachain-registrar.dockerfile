FROM node:latest 

RUN apt-get update && apt-get install curl netcat -y && \
    curl -sSo /wait-for-it.sh https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh && \
    chmod +x /wait-for-it.sh
COPY ./register/ /var/tmp/register
RUN cd /var/tmp/register && yarn && chmod +x index.js
# the only thing left to do is to actually run the transaction.
COPY ./register_para.sh /usr/bin
# unset the previous stage's entrypoint
ENTRYPOINT []
CMD [ "/usr/bin/register_para.sh" ]
