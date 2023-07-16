sh
jq -s 'map(map(select(.name == "def_span")) | map(.args.arg0) | sort) | (.[1]-.[0])' perf-{1,2}/chrome_profiler.json
