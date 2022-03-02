# Journal de bord
## 2021-12-09
Début du deuxième Travail de semestre

- Introduction au cours et ce qui est à faire côté documentation par M. Bonvin
- Analyse des tâches à réaliser pendant le cours de TSEM au deuxième semestre

But du projet:
Créer un client d'accès distant en Rust qui permet de se connecter à des protocoles variés, tel que le RDP, le VNC ou le XDMCP par exemple
## 2021-12-13
Discution du projet avec l'enseignant de suivi, M. Zeltner

Le cahier des charges aurait besoin d'un diagramme de séquences pour les actions utilisateurs et possiblement des précisions pour les actions utilisateurs décrites dans le doc.
## 2021-12-16
Ajout de précisions sur les rôles des modules dans l'analyse système
Travail sur la maquette de l'interface

## 2021-12-23
Installation de poste de travail Arch Linux

## 2022-01-13
Découverte de nouveaux frameworks [WebAssembly (WASM)](https://webassembly.org/) pour faciliter le développement et améliorer les performances de l'interface. Le WebAssembly est une solution possible pour remplacer les pages statiques afin d'avoir un rendu dynamique plus léger et rapide. 

WASM permet de faire des applications web avec des executables binaires intégrés. Ces éxecutables peuvent être compilés depuis le JS, ou cross-compilés depuis d'autres langues comme le C, C++, TypeScript, Rust, etc.

- [Yew](https://github.com/yewstack/yew)
- [Percy](https://github.com/chinedufn/percy)
- [Seed](https://github.com/seed-rs/seed)
- [Sycamore](https://github.com/sycamore-rs/sycamore)
- [MoonZoon](https://github.com/MoonZoon/MoonZoon)

## 2022-01-20
Début de la documentation technique et du rapport de stage

## 2022-01-27
Continuation de la documentation technique

## 2022-02-03
Debut du programmation du module NetworkMgr et supression du crate hostname. Ceci permet de reduire les dépendances et utiliser une commande de NetworkManager. La même commande avec un argument en plus set le hostname.

## 2022-02-10
Création du fonction load_profiles dans le module NetworkMgr permettant de charger les profiles de configuration stockées dans NetworkManager et permettant de déléguer le stockage des configuration réseau à ce dernier.

## 2022-02-24
Ajout de menu de reglages avec le systeme "sous-contenu" dans le JS et HTML, dans l'interface. Ceci permet un moyen dynamique d'ajoute des pages de réglages dans le HTML/JS. La prochaine étape serait de créer les formaulaires sur les differents pages de réglage.

## 2022-02-28
Ajout des composants "select" dans les pages de réglage "Network" et "Profiles" qui chargent les profiles réseau et profiles de connexion dans l'interface et permettent de selectionner le profile souhaité pour vision/édition

## 2022-03-02
Ajout des containeurs pour les forms de modifications de profiles de connexion et réseau