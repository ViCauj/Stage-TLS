import json 

with open("hash.json", "r") as f:
    merged_json = json.load(f)
with open("out.json", "r") as f:
    out_json = json.load(f)


print(merged_json)
print(out_json)


json_check = {"merged_json": merged_json, "signature": out_json}
with open("check.json", "w") as f:
    json.dump(json_check, f, indent=1)