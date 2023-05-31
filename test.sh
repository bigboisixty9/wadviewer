#!/bin/sh

export GIT_BRANCH=$(git symbolic-ref --short HEAD)
export GIT_COMMIT=$(git rev-parse HEAD)

trunk serve
