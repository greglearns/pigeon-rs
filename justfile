#!/usr/bin/env just --justfile
set shell := ["zsh", "-cu"]
set fallback

default:
  just -u --list

# set auth token
# export GOOGLE_OAUTH_TOKEN=$(oauth2l fetch --scope userinfo.email --scope gmail.send --credentials private/oauth2l_client_secret_171741523548-ur29cmomcqgbds37j0n8e56oukj51p49.apps.googleusercontent.com.json 2>/dev/null)

