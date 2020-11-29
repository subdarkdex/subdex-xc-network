#!/bin/sh

# this assumes you already have ssh-agent running locally and have accesss to 
# - server
# - github
docker-compose --file docker-compose-xc.yml down -v 
docker-compose --file docker-compose-traefik.yml down
docker volume prune
docker-compose --file docker-compose-xc.yml up -d 
docker-compose --file docker-compose-traefik.yml up -d 
