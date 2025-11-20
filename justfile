#!/usr/bin/env just --justfile
set shell := ["zsh", "-cu"]
set fallback

default:
  just -u --list

# set auth token
# export GOOGLE_OAUTH_TOKEN=$(oauth2l fetch --scope userinfo.email --scope gmail.send --credentials private/oauth2l_client_secret_171741523548-ur29cmomcqgbds37j0n8e56oukj51p49.apps.googleusercontent.com.json 2>/dev/null)

# send:
# cargo run -- send-bulk greg@yesyesconnect.com --connection google --db-url "${DB_URL}" --message-file "20251119-message-new-feature-1st-meeting-checklist.yaml" --receiver-query "SELECT * FROM human_email WHERE pid IN (4022,4024,4032,4041)" --display --personalize "name" --dry-run
