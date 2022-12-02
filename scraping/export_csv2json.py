import json
from random import randint

with open('C:/Users/trist/Downloads/Questions - Feuille 1.tsv','r',encoding='utf-8') as file:
    lignes = [ligne.split("\t") for ligne in file.read().split('\n')]

questions = []
tete = lignes[0]

for i in range(1,len(lignes)):
    print(lignes[i])
    print(len(lignes[i]))
    position = randint(0,1)
    intitulé = lignes[i][0]
    réponse = lignes[i][1]
    rep_1 = lignes[i][1 + position]
    rep_2 = lignes[i][2 - position]
    explication = lignes[i][3]
    src = lignes[i][4]
    cat = lignes[i][5]
    questions.append({"titre":intitulé,"reponse 1":rep_1,"reponse 2":rep_2,"reponse":réponse,"explication":explication,"source":src,"categorie":cat})

with open('resultats/questions.json','w',encoding="utf-8") as file:
    file.write(json.dumps(questions))
