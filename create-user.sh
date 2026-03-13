#! /bin/env bash

curl -X POST localhost:8080/user -H 'Content-Type: application/json' --data '{"username": "hallo"}'
