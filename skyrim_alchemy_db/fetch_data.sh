#!/usr/bin/env bash

curl https://alchemy-eight.vercel.app/api/ingredients | jq > ingredients.json
curl https://alchemy-eight.vercel.app/api/effects | jq > effects.json
