# Documentation technique
## Résumé / Abstract
Blackrust v0 est un logiciel multi-architecture pour linux qui, au lancement de la machine, proposera des sessions d'accès distant utilisant divers protocoles ainsi qu'une session locale hors-ligne.

Ce projet est un client léger qui a pour but de réduire la taille et le coût des moultes machines données aux employés dans une entreprise. Ces clients légers se connecteraient à un serveur central où résideraient les espaces de travail des utilisateurs avec d'avantage de puissance de calcul.

---

Blackrust v0 is a multi-architecture program for linux that, at the startup of the computer, will offer a remote desktop session via many protocols aswell as an offline local desktop.

This project is a thin client, which aims to reduce the size and cost of the many machines given to employees in a company. These thin clients will connect to a centralized server where the users workspace will be and will offer greater processing power.
## Cahier des charges
[Lien vers cahier des charges](index.md)
## Maquettes
## Développement
### Environnement de travail
L'environnement de travail utilisé lors du développement de ce projet consiste en:
- Ordinateur de l'école avec Arch Linux installé dessus,
- Visual Studio Code comme IDE
- Raspberry Pi 4
### Architecture
Le programme est décomposé en 5 modules principaux:
- Main: Point d'entrée du programme et aperçu graphique
- Config_Mgr: CRUD pour les options de connexion
- Network-Mgr: Appels aux outils système pour configurer le réseau
- RemoteSession_Mgr: Lanceur de sessions distant
- Blackrust-Lib: Fonctions commun a plusieurs modules, librairie interne
#### Modules
##### Main
##### Config_Mgr
##### Network_Mgr
##### RemoteSession_Mgr
##### Blackrust-Lib
#### Librairies externes
##### Web-view
##### Xrandr
##### Serde
##### Hostname
##### Image-base64
##### Regex
## Difficultés
## Planning
## Livrables
## Conclusion
## Bilan Personnel