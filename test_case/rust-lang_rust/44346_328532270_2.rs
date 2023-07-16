bash
if [ -f "$docker_dir/$image/Dockerfile" ]; then
    retry docker \
      build \
      --rm \
      -t rust-ci \
      -f "$docker_dir/$image/Dockerfile" \
      "$docker_dir"
elif [ -f "$docker_dir/disabled/$image/Dockerfile" ]; then
    if [ -n "$TRAVIS_OS_NAME" ]; then
        echo Cannot run disabled images on travis!
        exit 1
    fi
    retry tar --transform 's/^\.\/disabled\//.\//' -C $docker_dir -c . | docker \
      build \
      --rm \
      -t rust-ci \
      -f "$docker_dir/disabled/$image/Dockerfile" \
      -
else
    echo Invalid image: $image
    exit 1
fi
