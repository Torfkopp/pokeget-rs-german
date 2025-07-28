from bs4 import BeautifulSoup
import requests

html = requests.get("https://bulbapedia.bulbagarden.net/wiki/List_of_German_Pok%C3%A9mon_names")
soup = BeautifulSoup(html.content, "html.parser")

name_tuples = []

for tables in soup.find_all("table", class_="roundy roundtable"):
    for row in tables.find_all("tr"):
        cells = row.find_all("td")
        if len(cells) >= 2:
            dex_number = cells[0].get_text(strip=True)
            english_name = cells[2].get_text(strip=True)
            german_name = cells[3].get_text(strip=True)
            name_tuples.append((dex_number, english_name, german_name))

with open("data/german_names.txt", "w", encoding="utf-8") as file:
    for dex_number, english_name, german_name in name_tuples:
        file.write(f"{dex_number},{english_name},{german_name}\n")
