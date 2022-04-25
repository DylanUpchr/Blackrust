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

## 2022-04-06
Entretien avec M. Zeltner, sujet traités
- Revue du démarrage du programme
- Tâches à faire
- Planification prévisionnel
- Outil de gestion YouTrack 
- Planning du travail de diplôme

Définition des périmètres de tests unitaires
Documentation

## 2022-04-07
Gestion des erreurs dans le crate main en remplaçant des appels a unwrap() avec des match statements pour définir les actions à entreprendre lorsqu'une erreur survient.

## 2022-04-08
Gestion des erreurs dans le crate network_mgr en remplaçant des appels a unwrap() avec des match statements pour définir les actions à entreprendre lorsqu'une erreur survient.

Résolution du problème de panic lors du chargement de profile dans l'interface de réglages réseau. Le problème était que l'objet n'était pas sérialisé et donc je ne pouvais pas envoyer la propriété demandée. La solution était de parse le string JSON et récupérer la propriété souhaitée.

Suppression du test unitaire qui testait que la longueur de la liste d'interfaces réseau soit supérieur à 0, qui ne réussi pas car, le runner de Github Actions n'as pas d'interface réseau ou n'as pas NetworkManager installé. Solutions possibles: Faire un match statement sur le résultat et laisser le test paniquer et échouer si cela ne fonctionne pas. Le tests sont marqués comme échoué lors d'une panique donc une vérification pour un retour Err qui n'est pas implémenté n'est pas requis.

## 2022-04-11
Test unitaire de l'affichage avec un serveur X11 headless (alors un mock d'interface visuel / interface dummy/virtuel) sur le Github Runner. Sur un poste local le test réussi tout seul avec le serveur X11 réel, mais pour les tests faits en container CI un écran virtuel est requis. Le test vérifie que le WebView se construit et se lance sur un serveur X11.

## 2022-04-12
Ajout du crate rstest pour les "data-driven" tests unitaires, ceci permet de définir des cas de tests et alimenter les fonctions de tests avec des données afin de tester plusieurs scénarios.
Ajout, implémentation et documentation des tests unitaires:
- open_webview_test
- base64_encode_images_test
- exec_nmcli_command_test
- get_hostname_test
- set_hostname_test
Planification des tests unitaires pour le module network_mgr:
- get_all_interfaces_test
- get_interface_by_name_test
- load_all_profiles_test
- create_profile_test
- get_simple_profile_by_id_test
- get_detailed_profile_by_id_test
- modify_profile_test
- delete_profile_test

## 2022-04-13
Je rencontre un problème de droits révélé par le test unitaire pour set le hostname de la machine. Les droits définis par [Polkit](https://wiki.archlinux.org/title/Polkit) (org.freedesktop.NetworkManager.settings.modify.hostname) requière des droits administrateur et donc le programme ne peut pas changer le nom d'hôte de la machine locale. La solution est de définir la règle suivant avec un fichier .rule contenant suivant polkit copiée dans le répertoire ```/usr/share/polkit/rules.d/```:
```js
polkit.addRule(function(action, subject) {
    if (subject.user == "blackrust") {
        polkit.log("action=" + action);
        polkit.log("subject=" + subject);
        if (action.id.indexOf("org.freedesktop.NetworkManager.settings.modify.hostname") == 0) {
            return polkit.Result.YES;
        }
    }
});
```

## 2022-04-16
En implémentant les tests unitaires dans le module network_mgr, j'ai réalisé que dû au fait de l'exécution en parallèle des tests unitaires, je ne peux pas faire des actions séquentielles (comme ajout, modification et suppression d'un profile sans laisser de traces) avec l'outil de configuration de réseau. Ceci veut donc dire qu'il faudrait faire un mock de l'outil afin de tester les fonctions qui l'appèlent sans que l'outil affecte le système. 

J'ai pu faire cela avec le crate [mockall](https://docs.rs/mockall/latest/mockall/). Ce crate ressemble au paquet Moq pour C#, en qu'on peut créer des mocks de fonctions avec des paramètres extensibles, tels que les paramètres à la quel on s'y attend, combien d'appels on s'y attend et quel retour de la fonction on s'y attend. Ceci permet d'intercepter l'appel à la fonction de l'outil réseau et en même temps retourner une valeur ressemblante à la réalité afin de tester les transformations de données faites par les fonctions testées.

Afin de tester plusieurs cas sans redéfinir les tests à chaque fois j'ai utilisé le crate [rstest](https://docs.rs/rstest/latest/rstest/), qui permet de faire du Data-driven testing, donc de fournir des tableaux de données à la fonction de test et ce dernier prends ces données en paramètre qui nous permet de rendre les tests dynamiques propose un meilleur code coverage avec moins de fonctions de test.

Avec ces deux crates, j'ai implémenté la liste suivante de tests:
- exec_command_test: Test sans mock qu'un appel peut être fait à l'outil desirée
- get_hostname_test: Test avec mock pour récupérer le nom d'hote du système et pour la gestion d'erreur de ceci
- set_hostname_test: Test avec mock pour affecter le nom d'hote du système et pour la gestion d'erreur de ceci
- get_all_interfaces_test: Test avec mock pour récuperer les interfaces réseau du système et pour la gestion d'erreur de ceci
- get_interface_by_name_test: Test avec mock pour récupérer un interface réseau du système par son nom et pour la gestion d'erreur de ceci
- load_all_profiles_test: Test avec mock pour récupérer les profiles de connexion réseau du système et pour la gestion d'erreur de ceci
- create_profile_test: Test avec mock pour créer un profile de connexion réseau et pour la gestion d'erreur de ceci
- get_simple_profile_by_id_test: Test avec mock pour récupérer un profile de connexion réseau du système par son identifiant unique et pour la gestion d'erreur de ceci
- delete_profile_test: Test avec mock pour supprimer un profile de connexion réseau et pour la gestion d'erreur de ceci

Ce qui reste les tests suivants à faire dans le module network_mgr une fois que les fonctions sont correctement implémentées:
- get_detailed_profile_by_id_test: Test avec mock pour récupérer un profile de connexion réseau du système par son identifiant unique et pour la gestion d'erreur de ceci
- modify_profile_test: Test avec mock pour modifier un profile de connexion réseau et pour la gestion d'erreur de ceci

## 2022-04-22
Changed interface property to Option<Interface\> and changed get_interface_by_name return to match

Updated test documentation

