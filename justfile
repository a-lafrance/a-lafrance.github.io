deploy:
	#!/usr/bin/env sh
	CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

	if [ $CURRENT_BRANCH != master ]
	then
		echo "error: can only deploy from master"
		exit 1
	fi

	trunk build --release
	git subtree push --prefix=dist origin deploy
