import requests
from bs4 import BeautifulSoup as bs
import json


articles = []

soup = bs(requests.get("https://www.sexualites-info-sante.fr").text,"lxml")
arti = {}
for categorie in soup.find('ul',{'class':'wsp-posts-list'}).findAll('li'):
    try:
        cat = categorie.strong.text
        if not not arti:
            articles.append(arti)
        arti = {"titre":cat, "articles":[]}
    except:
        cat = categorie.text
        lien = categorie.a["href"]
        art_soup = bs(requests.get(lien).text,"lxml").find("div",{"class":"et-l et-l--post"})
        try:
            image = art_soup.find("img")["src"]
        except:
            image = None
        contenue = art_soup.find('div', {"class":"et_pb_text_inner"}).text
        arti["articles"].append({"titre":cat, "lien":lien, "image":image, contenue:contenue})
articles.append(arti)

print(articles[0])
with open('resultats/sexInSer_articles.json','w') as file:
    file.write(json.dumps(articles))
