
The command "if [ "$ALLOW_PR" = "" ] && [ "$TRAVIS_BRANCH" != "auto" ]; then

    echo skipping, not a full build;

elif [ "$TRAVIS_OS_NAME" = "osx" ]; then

    git submodule update --init &&

    src/ci/run.sh;

else

    git submodule update --init &&

    src/ci/docker/run.sh $IMAGE;

fi

" exited with 129.
