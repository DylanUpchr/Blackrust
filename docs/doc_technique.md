# Documentation technique
## Résumé / Abstract
Blackrust v0 est un logiciel multi-architecture pour linux qui, au lancement de la machine, proposera des sessions d'accès distant utilisant divers protocoles ainsi qu'une session locale hors-ligne.

Ce projet est un client léger qui a pour but de réduire la taille et le coût des moultes machines données aux employés dans une entreprise. Ces clients légers se connecteraient à un serveur central où résideraient les espaces de travail des utilisateurs avec d'avantage de puissance de calcul.

---

Blackrust v0 is a multi-architecture program for linux that, at the startup of the computer, will offer a remote desktop session via many protocols aswell as an offline local desktop.

This project is a thin client, which aims to reduce the size and cost of the many machines given to employees in a company. These thin clients will connect to a centralized server where the users workspace will be and will offer greater processing power.
## Cahier des charges
[Lien vers le cahier des charges](index.md)
## Maquettes
## Développement
### Environnement de travail
L'environnement de travail utilisé lors du développement de ce projet consiste en:
- Ordinateur de l'école avec Arch Linux installé dessus,
- Visual Studio Code comme IDE
- Raspberry Pi 4
### Architecture
#### Modules internes
Le programme est décomposé en 5 modules principaux:
- Main: Point d'entrée du programme et aperçu graphique
- ConfigMgr: CRUD pour les options de connexion sauvegardées
- NetworkMgr: Appels aux outils système pour configurer le réseau
- RemoteSessionMgr: Lanceur de sessions distant
- Blackrust-Lib: Fonctions commun a plusieurs modules, librairie interne
##### Main
Le module main est la point d'entrée principale de l'application, lance l'aperçu WebView qui permet d'interfacer avec l'application et appeler les autres modules
##### ConfigMgr
Le module ConfigMgr gère les profiles de connexion de session distant avec des fonctions CRUD (Création, Lecture, Mise à Jour, Suppression). Ses fonctionnalités sont appelé depuis le Invoke Handler du WebView et donc depuis le JS de l'interface utilisateur.
##### NetworkMgr
Le module NetworkMgr permet de faire des appels vers NetworkManager pour configurer les interface réseau afin de pouvoir se connecter au réseau local et éventuellement à un VPN.
##### RemoteSessionMgr
Le module RemoteSessionMgr lance les sessions distant en utilisant les options de connexion soit fourni par l'utilisateur soit par un profile chargé par l'utilisateur. Ce module fait appel aux commandes tel xfreerdp, vncviewer, Xnest ou ssh.
##### Blackrust-Lib
Blackrust-Lib est la libraire commun aux modules et contient les définitions de structures de données et le fonctions utilisé par tous les modules.
#### Librairies externes
Le programme utilise également quelques libraires externes, principalement pour la rendu graphique Web.
##### Web-view
Web-view est un crate qui agit en tant que navigateur web qui affiche le rendu HTML/CSS/JS
##### Xrandr
Xrandr permet de récuperer des informations sur le ou les écrans d'afficage, comme taille, DPI, disposition des moniteurs, etc.
##### Serde / Serde-JSON
Serde implémente des fonctionnalités de serialisation et déserialisation des instances d'objets vers et depuis le JavaScript Object Notation (JSON).
##### Hostname
Hostname permet de recupérer le nom d'hôte de la machine. **A remplacer par un appel a nmcli dans NetworkMgr**
##### Image-base64
Image-base64 est un crate qui encode ou "traduit" des fichiers image en texte base64. **Ceci est necessaire pour l'instant à cause de WebView qui ne peut pas referencer des fichier et que traiter du HTML pur. Ceci pourra changer en implementant Actix (Serveur Web) et Yew (Framework WASM pour Rust)**
##### Regex
Le crate Regex implémente des expressions regulières utilisées pour la verification des données saisies par l'utilisateur pour la configuration réseau
## Difficultés
- Appréhension de Rust
- WebView (Inclusion du CSS/JS et images encodées en base64)
- Définition des dépendance clef du projet à installer
## Planning
## Livrables
- Documentation
    - Cahier des charges
    - Journal de bord
    - Documentation technique
- Programme
    - Paquet avec scripts d'installation (PKGBUILD)
    - Code source (Lien Github)
## Conclusion
## Bilan Personnel