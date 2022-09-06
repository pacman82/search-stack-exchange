#!/bin/bash

set -euo pipefail

if [ $# -lt 1 ]; then
    echo "usage: $0 stackexchange-community" >&2
    exit
fi

if [ -f Posts.xml -o -f Tags.xml ]; then
    echo "Posts.xml or Tags.xml exist, delete them before running this script" >&2
    exit 1
fi

# use a temporary file for downloading to ensure files are complete once they exist in the current directory
tempfile=$(mktemp)
# delete temporary file on exit
trap "[ -e $tempfile ] && rm $tempfile" EXIT

# use curl instead of wget because it's more widely available
# TODO: use wget instead if available
echo "download $1 Posts.xml..." >&2
curl -sSL https://archive.org/download/stackexchange/"$1".stackexchange.com.7z/Posts.xml > $tempfile && mv $tempfile Posts.xml
echo "download $1 Tags.xml..." >&2
curl -sSL https://archive.org/download/stackexchange/"$1".stackexchange.com.7z/Tags.xml > $tempfile && mv $tempfile Tags.xml

# curl does not exit uncleanly when an HTTP 404 error page or something similar was provided
# do a quick-and-dirty integrity check by expecting a xml marker in the first line
for file in Posts.xml Tags.xml; do
    if ! head -n1 $file | fgrep '<?xml' >/dev/null; then
	echo "$1 is no valid community name, $file seems to contain the error page HTML" >&2
	exit 1
    fi
done
