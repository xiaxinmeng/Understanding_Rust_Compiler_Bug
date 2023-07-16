python
import json
with open("doc/mcve.json") as f:
    rdj = json.load(f)
idx = rdj["index"]
for item in idx.values():
    if item["kind"]=="struct" and item["name"] == "ArgMatches":
        impls = item["inner"]["impls"]
        break
if impls:
    exit(0)
else:
    exit(1)
