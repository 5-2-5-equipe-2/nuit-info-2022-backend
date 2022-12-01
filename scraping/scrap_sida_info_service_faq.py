import requests
from bs4 import BeautifulSoup as bs
import json


articles = []
for i in range(1,5):
    soup = bs(requests.get(f'https://www.sida-info-service.org/categorie/questions-frequentes/foire-aux-questions-sida/page/{i}/').text)

    for article in soup.findAll('article'):
        content = article.find("div",{"class":"entry-content"}).text.strip()
        tags = [a.text for a in article.footer.findAll("a")]
        articles.append({"titre":article.a.text, "résumé":content, "lien":article.a['href'], "catégories":tags})

with open('resultats/SIS_articles.json','w') as file:
    file.write(json.dumps(articles))
