deploy MSG:
	#!/usr/bin/env sh
	CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

	if [ $CURRENT_BRANCH != master ]
	then
		echo "error: can only deploy from master"
		exit 1
	fi

	trunk build --release

	git add .
	git commit -m "deploy: { MSG }"
	git push origin master
	git subtree push -f --prefix dist origin deploy
