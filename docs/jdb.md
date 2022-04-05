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
Début de la programmation du module NetworkMgr et suppression du crate hostname. Ceci permet de réduire les dépendances et utiliser une commande de NetworkManager. La même commande avec un argument en plus set le hostname.

## 2022-02-10
Création du fonction load_profiles dans le module NetworkMgr permettant de charger les profiles de configuration stockée dans NetworkManager et permettant de déléguer le stockage des configuration réseau à ce dernier.

## 2022-02-24
Ajout de menu de réglages avec le système "sous-contenu" dans le JS et HTML, dans l'interface. Ceci permet un moyen dynamique d'ajouter des pages de réglages dans le HTML/JS. La prochaine étape serait de créer les formulaires sur les ddifférents pages de réglage.

## 2022-02-28
Ajout des composants "select" dans les pages de réglage "Network" et "Profiles" qui chargent les profiles réseau et profiles de connexion dans l'interface et permettent de sélectionner le profile souhaité pour vision/édition

## 2022-03-02
Ajout des containeurs pour les forms de modifications de profiles de connexion et réseau avec affichage automatique selon le type de profile NetworkManager.

## 2022-03-05
Ajout du bouton création de profile réseau (avec fonction associée à compléter avec appel vers NM) et changement du checkbox "Automatic" pour le type de configuration ipv4/ipv6 d'un profile pour permettre plus de types de configuration comme Link-local et le fait de pouvoir ignorer le type d'adressage. Le bouton création crée un profile réseau vide dans NetworkManager et charge cette nouvelle profile dans l'interface pour modification. 

## 2022-03-08
Ajout des éléments dans l'interface ainsi que les appels aux fonctions (pas encore faites) qui permettent de créer des profiles de réseau NetworkManager et les profiles de connexion.

Ajout des fonctions pour récupérer les interface réseau depuis NetworkManager

## 2022-03-09
Ajout des bindings Rust et JS pour appeler les fonctions dans network_mgr pour récupérer les interfaces afin pouvoir affecter cela depuis l'interface utilisateur et d'avoir des objets "Interface" dans les profiles réseau.

## 2022-03-10
Terminé la création de profile (codé les fonctions définies)

## 2022-03-12
Changement du répertoire par défaut pour les configurations de /etc/blackrust/data/ à ~/.config/blackrust/data. Ceci permet à l'application de créer le répertoire si elle n'existe pas ainsi que garde les profiles de connexion de l'utilisateur sécurisé.

Ajout de vérifications que le chemin vers la quel on écrit le fichier qui contient les profiles de connexion existe, et sinon on crée le chemin et écrit dans le fichier afin de sauvegarder les profiles.

Terminé le CRUD (Ajout, modification de toutes les propriétés des struct et suppression des profiles) des profiles de connexion

CRUD basique pour créer les profiles réseau, modifier le nom et l'interface et supprimer le profile.

Le CRUD est maintenant assez fonctionnel pour affecter les profiles de connexion et l'assigner un profile réseau ce qui permet la connexion et le renseignement de nouveaux profiles de connexion vers des serveurs distants. 

## 2022-03-14
Ajout de l'appel de l'interface vers Rust pour la connexion à un serveur distant spécifié dans le profile de connexion. La prochaine étape serait d'implémenter une protocole vers la quel on peut se connecter.

Recherches sur le protocole XDMCP et création de la structure de classes nécessaires pour plusieurs implémentations de protocoles sous les modules remote_sessions_mgr et remote_protocols

Liens de recherche:

- [XDMCP Whitesheet](https://www.x.org/releases/X11R7.6/doc/libXdmcp/xdmcp.html)
- [Exemple de client XDMCP en C](https://github.com/astrand/xdmcpc)
- [XAuth Whitesheet](https://www.x.org/releases/current/doc/man/man1/xauth.1.xhtml)

Préparation documentation technique, mise à jour du journal de bord

## 2022-03-16
Avancements sur la documentation en anticipation du rendu finale du travail de semestre, ainsi que le rendu du cahier des charges pour le travail de diplôme

## 2022-03-17
Dernières modifications de la documentation avant le rendu à midi

## 2022-03-25
Exemples de CI/Versioning, projet Barrier KVM
[Repology](https://repology.org/project/barrier/versions)
[Github passed/failed versions](https://github.com/debauchee/barrier#distro-specific-packages)

## 2022-04-04
Début du travail de diplôme
- Présentation de M. Garcia et M. Bonvin le matin pour expliquer le déroulement et les règles du travail de diplôme
- Mise en place du poste de travail dans la salle R.105 pour assurer une meilleure répartition des élèves entre les deux salles
- Mise en place de l'outil de gestion de projet YouTrack

YouTrack est un outil fait par JetBrains comme Confluence fait. Il permet de la planification de projet Kanban/Agile, de la planification prévisionnel avec Gantt, des timesheets pour la planification effectif ainsi que créer un wiki pour le projet. L'outil permet également de créer des rapports sur tous ces éléments et donc facilite la création d'annexe en plus de la gestion de projet. 

Visite de l'HEPIA de 15h50 à 18h

## 2022-04-05
- Création des tâches essentielles dans YouTrack et planning prévisionnel
- Envoi d'un email à M. Zeltner, l'enseignant de suivi concernant le démarrage du projet et le planning prévisionnel
- Définition des tests de compatibilité et tests unitaires
- Mise en place du pipeline CI de Github Actions qui exécute cargo test.

Le pipeline Continuous Integration (CI) de Github Actions permet d'exécuter cargo test a chaque push vers Github. Github garde un log des tests exécutés et les erreurs/résultats obtenus. J'ai rencontré un problème avec les dépendances du projet qui n'étaient pas installé dans le container utilisé pour les tests unitaires. J'ai donc identifié et mis une étape qui installe les dépendances nécessaires suivantes afin de résoudre le problème :
(Format: nom de la dépendance (nom du paquet ubuntu))
- Pango (librust-pango-dev)
- ATK (librust-atk-dev)
- Soup (libsoup2.4-dev)
- Webkit2GTK (libwebkit2gtk-4.0-dev)
- NPM (npm)
