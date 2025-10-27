#!/bin/bash

docker container exec konsol-2024-backend-1 python3 add_authenticated_user.py "$@" /app/data/db/konsol.db