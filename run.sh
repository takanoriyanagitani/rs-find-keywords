#!/bin/sh

export ENV_MATCH_KIND=left-most-first
export ENV_MATCH_KIND=first
export ENV_MATCH_KIND=standard
export ENV_MATCH_KIND=std
export ENV_MATCH_KIND=left-most-longest
export ENV_MATCH_KIND=long

export ENV_INVERT_MATCH=false

export ENV_WRITE_ID=true
export ENV_WRITE_ID=false

echo helo wrld | ./rs-find-keywords helo
echo helo wrld | ./rs-find-keywords wrld
echo helo wrld | ./rs-find-keywords helo wrld
echo helo wrld | ./rs-find-keywords hello
echo helo wrld | ./rs-find-keywords world
