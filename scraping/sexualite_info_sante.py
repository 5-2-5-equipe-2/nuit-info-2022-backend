import requests
from bs4 import BeautifulSoup as bs
import json


articles = []

soup = bs(requests.get("https://www.sexualites-info-sante.fr").text)
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
        arti["articles"].append({"titre":cat, "lien":lien})
articles.append(arti)

print(articles)
##for article in soup.findAll('article'):
##    content = article.find("div",{"class":"entry-content"}).text.strip()
##    tags = [a.text for a in article.footer.findAll("a")]
##    articles.append({"titre":article.a.text, "résumé":content, "lien":article.a['href'], "catégories":tags})

##print(len(articles))
##with open('articles.json','w') as file:
##    file.write(json.dumps(articles))
