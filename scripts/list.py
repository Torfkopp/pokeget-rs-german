import csv

with open('data/pokemon.txt', 'r') as file:
    german_names = file.read().splitlines()

with open('data/list.csv', 'w', newline='') as file:
    writer = csv.writer(file)
    
    for i, name, gname in (x.split(",") for x in german_names):
        filename = name.lower().replace(' ', "-").replace('_', '-').replace('.', "").replace('\'', "").replace(":", "")
        writer.writerow([name, gname, filename])

    
