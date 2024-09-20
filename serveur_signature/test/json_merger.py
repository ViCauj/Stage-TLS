import json 

n = 3
json_liste = ["hash" + str(i) + ".json" for i in range(1, n+1)]
json_obj = []

for fichier in json_liste:
    with open(fichier, "r") as f:
        data = json.load(f)
        json_obj.append(data)

json_merged = {"obj": json_obj}
with open("hash.json", "w") as f:
    json.dump(json_merged, f, indent=1)