# Rapport du Travail de diplôme
## Résumé / Abstract
Blackrust est un logiciel multiarchitecture pour Linux qui, au lancement de la machine, propose des sessions d'{{ lex("accès distant", "L'accès distant, aussi nommé session distante ou desktop distant, est la prise de contrôle du bureau d'un ordinateur dans un autre emplacement via le réseau.") }} sécurisées utilisant de divers {{ lex("protocoles", "Un protocole distant dans le contexte de mon projet est un langage définie que les différents types de serveurs d'accès distant utilisent pour communiquer avec les programmes qui servent de client.") }} ainsi qu'une session locale hors-ligne.

Ce projet est un client léger qui a pour but de réduire la taille et le coût de moult machines données aux employés dans une entreprise avec de l'infrastructure VDI (virtualisation du poste de travail). Ces clients légers se connecteront à un serveur central où résideront les espaces de travail des utilisateurs avec davantage de puissance de calcul que sur la machine locale.

---

Blackrust is a multi-architecture program for Linux that, at the startup of the computer, offers a secure remote desktop session via many protocols as well as an offline local desktop.

This project is a thin client, which aims to reduce the size and cost of the many machines given to employees in a company using {{ lex("VDI", "VDI, ou Virtual Desktop Infrastructure est un type d'infrastructure compris de machines virtuelles qui servent d'espaces de travail à utiliser en accès distant.") }} (virtual desktop infrastructure). These thin clients will connect to a centralized server where the user's workspace resides and offer greater processing power than the local machine.
## Introduction
Blackrust permet de prendre la main sur des ordinateurs à distant en utilisant de divers protocoles d'accès distant afin de pouvoir proposer le plus de compatibilité que possible avec les systèmes distants. Elle permet également d'ouvrir un bureau normal sur la machine locale si une session distante n'est pas souhaitée par l'utilisateur.

L'application propose une interface Web pour interagir avec le backend Rust qui permet de configurer le système local, et se connecter à des systèmes distants. Les connexions aux machines cibles peuvent être sécurisées avec un VPN. L'application propose la fonctionnalité de se connecter à un VPN qui a déjà été mis en place au préalable.

<div style="page-break-after: always;"></div>
Voici un diagramme démontrant l'architecture de réseau généraliste, où le client Blackrust (représenté en bas du diagramme) se connecte à des clients soit Windows, Linux, macOS ou autre (ces derniers représentés en haut de l'image) dépendant uniquement du protocole utilisé entre les deux. La connexion se fait de préférence par une connexion sécurisée (représentée au centre de l'image) dépendant de l'environnement à disposition de l'utilisateur.

{{ fig("/img/network_diagram.svg", " Architecture de réseau généraliste") }}

Cette interopérabilité avec les différents environnements distants est due à la diversité de protocoles pris en charge par l'application. Des exemples de ces divers protocoles sont: 
- RDP fait par Microsoft pour prendre en charge les bureaux distants Windows
- VNC pour les hôtes macOS / Windows / Linux, ou autres machines où un serveur VNC est disponible.
- XDMCP spécifiquement pour les hôtes Linux avec un serveur d'affichage X11. 
- SSH X11-Forwarding pour une connexion limitée à une application graphique distante via le SSH

Ces protocoles sont tous multi-utilisateur, donc adaptés à l'utilisation dans un environnement de VDI

Le frontend de l'application est une page Web, soit afficheé en local uniquement soit mise à disposition sur un réseau en tant que serveur Web. La page Web peut ensuite communiquer avec le backend Rust par le biais des routes API mises à disposition par le serveur Web. Ceci permet d'avoir un seul backend pour un ou plusieurs clients et de séparer "l'intelligence" entre le backend et le front-end. Ainsi une application web dans n'importe quel langage pourrait interfacer avec le backend de l'application qui augmente l'extensibilité de l'application.

L'application Web du frontend est une application Yew, qui est un framework Web qui utilise un système de composants comme React ou Elm en Javascript. La principale différence est que Yew compile entre deux langages (ou transpile) du code Rust vers le WebAssembly. Ceci permet de gros gains de performances par rapport au JS, ainsi que des gains de sécurité grâce à la sûreté de la mémoire en Rust. Le code WebAssembly transpilé depuis le Rust est fourni au serveur Actix Web qui gère aussi les routes API. L'API et l'application peuvent être hébergés uniquement sur la machine en local ainsi que sur un réseau pour que plusieurs machines puissent utiliser une instance de l'application.

Le backend Rust est composé d'un système de sauvegarde/modification de configuration de connexion, un système de configuration réseau et un système de gestion de connexion. Voici un diagramme démontrant cela. Le backend utilise une librairie interne nommée BlackrustLib, qui met à disposition à tous le modules des fonctions ou structures de données.

{{ fig("/img/Main_data_flow.png", " Main data flow", 85) }}

Le système de configuration permet de gérer les profils de connexion mémorisés qui sont utilisés pour créer des sessions distantes selon une configuration prédéfinie. Ces profils sont sauvegardés dans un fichier de données sérialisées en utilisant le langage de markdown TOML. 
Ce module se repose sur deux fonctions de base: la sérialisation et déserialisation TOML d'objets de profil de connexion. Le reste des fonctionnalités (création, recherche, modification, suppression) découlent de ces derniers et sont mises à disposition par l'API Actix.

Le système de configuration réseau communique avec l'outil tiers NetworkManager afin de pouvoir proposer la possibilité d'affecter la configuration réseau du système local et sauvegarder plusieurs configurations réseau et configurations VPN.
Ce module utilise la commande en ligne de commande ```nmcli``` pour interfacer avec l'outil de configuration réseau NetworkManager. Toutes les fonctionnalités implémentées font usage des différents arguments de cet outil afin de proposer les fonctionnalités nécessaires pour la configuration réseau à travers l'interface utilisateur.

Le système de gestion de connexion s'occupe de lancer et gérer des sessions distantes en utilisant soit les profils prédéfinis, soit la saisie utilisateur. La session est lancée dans un affichage X11 "headless" qui est mis à disposition dans l'interface Web grâce à un serveur VNC local. Le système de gestion de connexion ouvre des sessions distantes avec l'aide d'outils déjà existants en ligne de commande, ou dans le cas de XDMCP, la négociation est faite manuellement afin d'augmenter la complexité du projet ainsi qu'approfondir mes connaissances personnelles de X11 ainsi que le protocole XDMCP.

Finalement la librairie interne met à disposition les fonctions d'écriture de fichiers utilisés par le module ConfigMgr ainsi que les structures de données pour les profils de connexion et les profils de connexion réseau utilisés par les modules ConfigMgr, NetworkMgr et RemoteSessionMgr.

L'architecture de la partie interface homme-machine, ou {{ lex("IHM", "Interface Homme-Machine, est la partie d'une application qui permet à l'utilisateur final d'interagir avec l'application.") }}, permet de proposer cette application en tant que client logiciel sur une machine indépendante, ainsi qu'en tant que serveur web, proposant les fonctionnalités à tous appareils équipés d'un navigateur. Ceci est dû au fait que l'interface Web délègue tout traitement à un backend, qui peut être disponible uniquement en local ou derrière un serveur web qui héberge une application web conçue pour ce cas d'utilisation.

Tous ces modules fonctionnent en tandem afin de proposer un client d'accès distant multiprotocole.
## Planning
### Prévisionnel
Le planning prévisionnel a été établi avec la fonctionnalité Gantt de l'outil YouTrack que j'utilise pour la gestion du projet. J'ai choisi de faire avec cet outil, car, je peux générer de divers types de rapports sur les tâches accomplies et le temps que ces derniers ont pris.

{{ fig("/img/planning_previsionnel.png", " Planning prévisionnel", 85) }}
### Effectif

<div style="page-break-after: always;"></div>

## Analyse de l'existant
Il existe déjà plusieurs solutions pour l'accès distant multiprotocole.
Celles-ci sont la plupart du temps limités en termes de compatibilité avec les différentes architectures système et sont soit propriétaires et/ou payantes.

La valeur ajoutée proposée par ma solution est que ceci est léger, sécurisé, multiplateforme et rapide, permettant de tourner sur des machines avec peu de puissance de processeur, laissant de la puissance pour le décodage du flux vidéo de la session distante afin d'offrir une bonne expérience utilisateur. Ma solution est également open source, gratuit et multiplateforme. Ceci permet l'accès ouvert et de l'extensibilité pour les utilisateurs avec des capacités de développement Rust.

Ce qui distingue Blackrust encore plus des autres prestations est son concept. Les autres applications sont des applications desktop faits pour êtres lancés une fois qu'on est déjà connecté sur une session locale, alors que Blackrust se lance avant la session locale et est un Display Manager / client distant. Cela veut dire que l'utilisateur a la possibilité d'uniquement lancer une ou plusieurs sessions distantes ou locales.

Le cahier des charges contient une analyse concurrentielle des autres solutions d'accès distant similaires afin de pouvoir se positionner par rapport à eux.

## Cahier des charges
[Lien vers le cahier des charges](1_cdc.md)

## Librairies / outils
### Librairie interne
#### Blackrust-Lib
Blackrust-Lib est la libraire commune aux modules et contient les définitions de structures de données et les fonctions utilisées par tous les modules.

### Librairies externes
Le programme utilise également quelques libraires externes écrites en Rust, principalement pour le rendu graphique Web.
Ces librairies externes (ou {{ lex("crates", "Librairie ou paquet Rust externe au projet. Un crate peut provenir depuis crates.io ou depuis un repository git.") }}) proviennent de [crates.io](https://crates.io), le repository de crates faites par la communauté Rust.
#### Web-view
Web-view est un crate qui agit en tant que navigateur web qui affiche le rendu HTML/CSS/JS.
#### Xrandr
Xrandr permet de récupérer des informations sur le ou les écrans d'affichage, comme taille, DPI, disposition des moniteurs, etc.
#### Serde / Serde-JSON
Serde implémente des fonctionnalités de sérialisation et désérialisation des instances d'objets vers et depuis le JavaScript Object Notation (JSON).
#### TOML
Le crate TOML est un sérialiseur/déserialiseur de TOML, Tom's Obvious, Minimal Language, qui est le langage de markdown utilisé par Rust pour représenter des données
#### Itertools
Le crate Itertools propose davantage de fonctions d'opérations sur les itérables
#### Dirs
Le crate Dirs sert à récupérer des chemins utilisateur selon l'utilisateur, comme le répertoire de fichiers de configuration ou le répertoire home.
#### RSTest
Le crate RSTest est un framework de test qui propose des tests unitaires "Data-Driven"
#### MockAll
Le crate MockAll est un framework de test qui permet de simuler des classes qui implémentent des traits.
#### Xvnc
La commande Xvnc est utilisée pour instancier un affichage headless et un serveur VNC qui met cet affichage à disposition sur la machine locale
#### NetworkManager
L'outil NetworkManager, utilisé avec la commande nmcli, traite toute la configuration réseau locale/VPN
#### Actix Web
Actix Web est une librairie de serveur web. Elle permet de créer et héberger un serveur HTTP/HTTPS avec une page web et/ou un API REST. Je l'utilise pour héberger l'application Web HTML/JS/CSS/WASM construite par Yew.
#### Yew
Yew est un framework Web qui permet de créer une application Web composée de fichiers HTML/JS/CSS/WASM. L'intelligence et la logique métier dans l'application est exécuté en WebAssembly, qui est un nouveau type d'exécutable haute-performance conçu pour le navigateur.

Yew ressemble à des frameworks JS tels que React ou Elm, avec leurs systèmes de composants. La principale différence est que Yew compile entre deux langages (ou transpile) du code Rust vers le WebAssembly.

### Protocoles
#### XDMCP
Le protocole X Display Manager Control Protocol (XDMCP) est un protocole d'accès distant qui permet de recevoir la sortie d'un affichage X11, ainsi qu'envoyer des touches clavier vers cette même session X11 distante. XDMCP est un protocole très simple et bien documenté, qui fait que c'est très faisable de faire sa propre client pour négocier une session avec.
#### RDP
Le protocole Remote Desktop Protocol (RDP) est un protocole d'accès distant pour Windows fait par Microsoft. Ce dernier est très riche en fonctionnalités, mais propriétaire et closed source, qui signifie que nous n'avons pas le code source derrière et hors des outils officiels ou reverse-engineered, il n'existe pas de documentation pour l'implémenter manuellement dans l'application et la négociation de ceci doit être délégué à un outil existant. 
#### VNC
Le protocole Virtual Network Computing (VNC) est un protocole d'accès distant multiplateforme et open source. C'est un protocole très polyvalent et bien documenté qui fait qu'il existe plusieurs dérivés en plus de la version officielle de RealVNC. Il existe plein d'outils déjà faits ainsi que la documentation et code source nécessaire pour le réimplémenter manuellement.
#### SSH X11-Forwarding
Le protocole SSH X11-Forwarding permet de faire tourner une application graphique sur une session X11 distante et récupérer l'affichage / interagir avec l'application via une connexion SSH. Ceci est utile quand on souhaite uniquement avoir accès à une application distante en ayant accès sur le bureau actuel.

## Normes
### Nommage
Rust impose le snake case (exemple_nom) pour les noms des fonctions et des variables et pascal case (ExempleNom) pour le nom des objets.
### Commentaires
Les fichiers ont comme entête le suivant :
```
/** File
 * Author:		Dylan Upchurch
 * Date:		2022-01-01
 * Desc:		File purpose
 */ 
```
#### RustDoc
RustDoc permet de générer des pages html de documentation automatiquement selon les commentaires mises dans les fichiers. Un site est généré à partir du code ainsi que des entêtes markdown mises dans le code à coté de ce qu'il documente.

Les fonctions sont précédées par un entête comme le suivant :
```
/// Ce que fait la fonction
///
/// Ce que retourne la fonction
///
/// # Examples
///
/// ```
/// exemple d'utilisation
/// ```
```

Les structs sont précédés par un entête comme le suivant :
```
/// A quoi sert le struct
```
Les champs des structs sont ensuite précedés par un commentaire comme le suivant :
```
/// A quoi sert le champs
```
Les enums sont précédés par un entête comme le suivant :
```
/// A quoi sert le enum
```
Les membres des enums sont précédés par un commentaire comme le suivant :
```
/// A quoi sert le champs
```

### Commits
Les messages de commits s'agissent d'un commentaire descriptif bref en anglais qui explique ce que contient le commit. Les différentes actions sont séparées par des virgules. Exemple de message de commit : ("Added functionnality X, removed unused code")

## Organisation
### Parties prenantes
| Nom | Rôle |
|-|-|
| Dylan Upchurch | Élève |
| Yannick Zeltner | Enseignant de suivi |

### Outil de gestion
La gestion du projet se fait avec l'outil YouTrack. Ce dernier propose des fonctionnalités Gantt, Kanban, relevée d'horaires et de génération de rapports sur ces derniers. 

#### Gantt
#### Kanban
#### Rapports

## Environnement de travail
L'environnement de travail utilisé lors du développement de ce projet consiste en :

### Matériel
- Ordinateur de l'école "upchr-arch"
- Raspberry Pi Model 4B (4GB) "Testbed-Rpi"
- Jetson Nano Developer Kit "Testbed-JN"
### Software
#### Arch Linux (upchr-arch)
- Visual Studio Code
- Rust "Stable"
#### Raspbian / Debian 11 Bullseye (Testbed-Rpi)
- Blackrust
#### Arch Linux (Testbed-Rpi)
- Blackrust
#### Linux4Tegra (Testbed-JN)
- Blackrust

## Choix du langage
J'ai choisi Rust comme langage pour le travail de semestre, car c'est un langage moderne. Rust est connu pour sa fiabilité, sécurité et rapidité.

### Rapidité
Rust est connu pour sa rapidité grâce à certains caractéristiques :
- Rust est statiquement typé, donc après la vérification de cargo check, pleins de vérifications au runtime peuvent être sautées
- Rust n'a pas de Garbage Collector, la mémoire est allouée et libérée selon "l'espérance de vie" d'une variable et donc cette dernière existe aussi longtemps que nécessaire. Ceci réduit les ressources consommées par un Garbage Collector et enlève les tâches répétitives de gérance de mémoire manuelle.
- Rust utilise le LLVM pour générer du code assembly optimisé, qui est comparable au GCC en termes de performances du programme final
### Compilateur
L'outil de compilation de Rust, nommée cargo, a plusieurs rôles :
- Package manager, pour les "crates" qui sont les paquets/modules officiels et de la communauté
- Validateur du code, cargo check vérifie plusieurs aspects avant de compiler le programme :
    - Que la gérance du mémoire est bien faite et ne viole pas les règles d'appartenance ou d'emprunt de références
    - Que les variables sont nommées en snake case, sinon il affiche des warnings
    - Qu'il n'y a pas du code "mort", donc pas utilisé, sinon il affiche des warnings
- Compilateur, bien entendu si le code ne contient pas d'erreur de syntaxe ni de gérance de mémoire le programme est compilé et rend un exécutable dans le dossier target

Les messages d'erreurs de cargo sont assez riches comparés aux autres langages. Cargo peut décrire l'erreur détectée en détail et même selon le type d'erreur, il peut suggérer des solutions. Si cela ne suffit pas, le traçage de la pile d'appels est accessible et peut aider avec le débogage traditionnel.
### Sécurité / Fiabilité
De base, le langage Rust est assez sécure et fiable grâce aux faites suivantes :
- Rust est "memory-safe", qui signifie qu'il ne permet pas d'avoir des pointeurs null ou invalide
- Les courses de données sont également impossibles, grâce au système d'appartenance, qui impose qu'une instance ou référence variable ne puisse être modifiée dans dans un endroit à la fois, afin d'éviter des "courses de données".
- La gestion d'erreur est très avancée et devrait être au cœur de la conception d'une fonction. Cette approche permet d'être toujours certain que le déroulement se passe comme prévu et les cas de bords qui pourraient compromettre la sécurité de l'application sont évités.
- Fonctionnalités de tests unitaires intégrées

### Tests unitaires
Rust contient une suite de tests unitaires permettant de fiabiliser le développement continu. Les tests sont des fonctions marquées avec un flag ```#[test] ``` et exécutées avec l'outil interne ```cargo test```.

### Multiplateforme
Rust est un langage avec un compilateur portable comme le langage C, donc qui peut être compilé sur la plupart des plateformes avec certaines garanties de fonctionnalité. Rust catégorise ces garanties dans un système de tiers. Les tiers sont ainsi :

- Tier 1: Garantie d'exécution, un programme en Rust pure est capable de compiler et de s'exécuter sans problèmes
    - Exemples : x86_64 Windows, x86_64 MacOS, x86_64 Linux, AArch64 Linux (ARM64)
- Tier 2: Garantie de compilation, un programme en Rust pure est capable d'être compilé, mais n'a pas une garantie 100% de fonctionner parfaitement lors de l'exécution
    - Exemples: iOS, Android, RISC-V, MIPS/MIPS64, PowerPC/PowerPC64 
- Tier 3: Pas de garanties de compilation ni d'exécution, mais ont une possibilité de fonctionner et pour certains des programmes ont déjà été faites
    - Exemples : Apple tvOS, Nintendo 3DS, CUDA, EFI

#### Technologies utilisées
##### WebView
Webkit est un moteur de navigateur développé par Apple parmi d'autres. Le moteur est utilisé par de diverses applications grâce à son API C++ qui propose des fonctionnalités pour afficher du contenu web dans une fenêtre avec des fonctionnalités de navigateur commun comme un historique ou la possibilité de retourner en arrière / aller en avant dans la navigation.

Je l'utilise pour l'interface utilisateur qui est une interface Web qui peut communiquer avec le programme Rust.
##### TOML
TOML, ou Tom's Obvious Markdown Language est le langage de markdown pour la sérialisation de données choisi par les développeurs de Rust.

Les fichiers TOML sont utilisés pour stocker les profils de connexions dans le répertoire de configuration par défaut de l'utilisateur
##### Github Actions
Github Actions permet d'exécuter dans un environnement sain les tests unitaires lors de chacun des push vers Github. Cela me propose un historique de tous les résultats de tests et permet d'avoir un pipeline d'intégration continu.
##### X11
X11 est le serveur d'affichage utilisé pour afficher le programme sur l'écran de l'utilisateur, mais encore peut être utilisé comme serveur d'affichage distant, soit par négociation XDMCP ou par SSH avec le X11-Forwarding
##### NetworkManager
NetworkManager est l'outil de configuration réseau utilisé par mon programme. Cet outil peut configurer, stocker et activer des profils réseau afin de pouvoir dynamiquement se connecter aux différents réseaux locaux ou VPNs selon le profil de connexion réseau choisi.
##### Polkit
Polkit, ou PolicyKit, est un gestionnaire de droits au sein de Linux. Je l'utilise pour donner certains accès à l'utilisateur à la configuration réseau, car certaines commandes NetworkManager nécessitent l'authentification.
###### XDMCP
Le protocole distant XDMCP est un des moyens de connexion pour mon application. Elle permet de négocier une connexion entre un serveur X11 local et un autre distant.
###### RDP
Le protocole distant RDP est un des moyens de connexion pour mon application. C'est un  protocole développé par Microsoft pour l'accès distant sur Windows.
J'exploite ce protocole avec l'outil xfreerdp qui est un client RDP Open-Source fait par la communauté grâce au reverse engineering.
###### VNC
Le protocole distant VNC est un des moyens de connexion pour mon application. J'exploite ce protocole avec l'outil vncviewer de RealVNC.
###### SSH X11-Forwarding
Le protocole distant SSH X11-Forwarding est un des moyens de connexion pour mon application. Elle permet de lancer des applications graphiques sur une session X11 distante, et avoir l'affichage en local par le biais d'une connexion SSH.

## Analyse fonctionnelle
L'analyse fonctionnelle définit les fonctionnalités de l'application ainsi que des explications sur les parties de l'interface utilisateur qui permet de les exploiter.
### Maquettes
#### Page principale de connexion

{{ fig("/img/home_component.svg", " Home page", 85) }}
La page d'accueil est le menu utilisé pour se connecter à session distante. Les sessions ouvertes sont ensuite ouvertes et affichées, accessibles depuis des onglets.
Voici des explications pour les points associés sur l'image:

- 1: Formulaire de connexion rapide/connexion à un profil
    - 1A: Menu déroulant pour choisir le protocole de la connexion rapide (RDP, VNC, XDMCP, SSH ou session locale)
    - 1B: Menu déroulant pour choisir le protocole de port (TCP ou UDP)
    - 1C: Champs de saisie pour l'adresse IP ou FQDN du serveur distant
    - 1D: Bouton pour ouvrir/fermer le menu déroulant pour choisir les profils de connexion sauvegardés 
    - 1E: Bouton qui lance la connexion
    - 1F: Bouton qui ouvre un champ de saisie qui permet d'ajouter des connexions spécifiques à la connexion
- 2: Nom d'hôte de la machine
- 3: Heure actuelle
- 4: Bouton pour atteindre les menus de réglage
- 5: Bannière personnalisable
- 6: Fond d'écran personnalisable
- 7: Barre d'onglets de session
    - 7A: Onglet menu principal
    - 7B: Onglet Session X avec un bouton pour fermer et se déconnecter de la session

#### Template de page de réglages
{{ fig("/img/settings_component_template.svg", " Settings page template", 85) }}
Le menu de réglages contient plusieurs sous-menus de configuration.
Voici des explications pour les points associés sur l'image:

- 1: Sous-menus de configuration
    - 1A: Sous-menu de réseau
    - 1B: Sous-menu de profils de connexion
    - 1C: Sous-menu de thème
    - 1D: Sous-menu d'internationalisation, donc de langue et de région pour les formats d'affichage de date/heure
    - 1E: Sous-menu "About" qui contient des informations sur l'application
- 2: Nom du sous-menu
- 3: Formulaire du sous-menu
- 4: Bouton pour fermer la page de réglage et retourner vers la page d'accueil

### Fonctionnalités de l'application
#### Connexion rapide
Un utilisateur a la possibilité de se connecter sans sauvegarder de profil de connexion. La marche à suivre est le suivant:

- Renseigner la configuration réseau avec la page de réglages prévue à cet effet
- Renseigner le protocole utilisé, ainsi que l'IP et le port de la machine distante dans la page de connexion
- Appuyer sur le bouton de connexion
Une fois ces manipulations faites, un onglet avec la session distante s'ouvre et est affiché.
#### Création/Modification/Suppression de profil de connexion
Un utilisateur a la possibilité de gérer ses connexions sauvegardées en utilisant la page de réglage prévu à cet effet. Ce dernier permet les fonctionnalités suivantes:

- Création de profil de connexion
- Modification des paramètres du profil de connexion précédemment crée
- Suppression d'un profil de connexion qui n'est plus souhaitée par l'utilisateur
#### Création/Modification/Suppression de profil de configuration réseau
Un utilisateur a la possibilité de gérer ses configurations réseau sauvegardées en utilisant la page de réglage prévu à cet effet. Ce dernier permet les fonctionnalités suivantes:

- Création de profil de connexion réseau
- Modification des paramètres du profil de connexion réseau précédemment crée
- Suppression d'un profil de connexion réseau qui n'est plus souhaitée par l'utilisateur
Ces actions entrainent des appels à l'outil réseau utilisé par le système afin de déléguer l'affectation des réglages système.
#### Connexion à un profil de connexion
Un utilisateur a la possibilité de se connecter à un serveur distant en utilisant un profil de connexion précédemment renseigné. La marche à suivre est le suivant:

- Choisir le profil de connexion souhaitée dans le composant barre de recherche/menu déroulant prévu à cet effet
- Appuyer sur le bouton de connexion
Une fois ces manipulations faites, un onglet avec la session distante s'ouvre et est affiché.
#### Basculer vers un *n* ème session ouverte ou page de connexion
Un utilisateur a la possibilité d'ouvrir plusieurs sessions et lors de l'usage de l'application, afficher la session qu'il désire utiliser en utilisant le système d'onglets prévu à cet effet.
Une barre d'onglets est affichée sur le haut de l'écran contenant les onglets qui représentent les sessions, ainsi que l'onglet qui représente la page de connexion permettant à l'utilisateur de naviguer l'application.

<div style="page-break-after: always;"></div>

## Analyse organique
### Architecture
{{ fig("/img/blackrust-systems-analysis.svg", " Analyse système", 85) }}
Le programme est décomposé en 5 modules principaux :

- Main (DM): Point d'entrée du programme et aperçu graphique
- ConfigMgr: CRUD pour les options de connexion sauvegardées
- NetworkMgr: Module qui configure le réseau (IPv4, IPv6, configuration VPN) à travers la commande `nmcli` de [NetworkManager](https://networkmanager.dev/)
- RemoteSessionMgr: Module qui lance et gère de sessions distantes
    - RemoteProtocols 
        - XDMCP
        - VNC
        - RDP
        - SSH
- BlackrustLib: Fonctions communes à plusieurs modules, librairie interne
#### Main
Le module main est le point d'entrée principale de l'application, lance l'aperçu WebView qui permet d'interfacer avec l'application et appeler les autres modules

{{ fig("/img/blackrust-systems-analysis.svg", " Analyse crate Main", 85) }}
##### Data flow
Le diagramme suivant détaille le dataflow du crate Main et représente graphiquement l'interaction entre l'utilisateur et les différents modules.
L'utilisateur final interagit avec l'interface Web mise à disposition par le moteur Webkit qui propose une sorte de navigateur appelé Webview. Cette interface Web communique ensuite bilatéralement avec le invoke handler de la partie "Backend" du Webview, qui est écrit en Rust. Le invoke handler expédié les différents appels vers les modules appropriés et rappel des fonctions JS avec le résultat si cela est nécessaire. Les modules Rust utilisent tous des modules de la librairie interne "BlackrustLib" représentée sur la droite du diagramme. Les modules de la librairie interne contiennent des définitions de types et des fonctions communes à tous les modules principales.

{{ fig("/img/Main_data_flow.png", " Main data flow", 85) }}
##### Fonctions
```start_actix```: Instancie et démarre le Serveur web Actix

<hr />

Type de retour

|Type|Description|
|-|-|
|io::Result<()>|Résultat qui rend une variante Ok vide|

##### Routes API Actix
Le serveur web Actix propose la fonctionnalité de définir un API qui peut appeler des fonctions Rust. Je l'utilise pour mettre à disposition des fonctions des différents modules pour que les client Web puisse les appeler afin qu'il puisse interagir avec le backend Rust.

Les routes sont groupées dans des scopes qui leur donne une préfixe à leur URL. J'ai choisi de les grouper par module concerné, donc NetMgr pour NetworkMgr, CfgMgr pour ConfigMgr et RsMgr pour RemoteSessionMgr.

###### NetMgr Scope
Ce scope expose les routes qui concernent la gestion du configuration réseau.
Le préfixe du scope NetMgr est ```/net_mgr``` qui donne ```/net_mgr/hostname``` par exemple.

| Nom du service | Méthode | Route | Fonction appelé | Structure du payload (optionnel) |
|-|-|-|-|-|
|get_hostname| GET | ```/hostname``` | ```network_mgr::get_hostname``` |
|set_hostname| PUT | ```/hostname``` | ```network_mgr::set_hostname``` | HostnameFormData |
|get_net_profiles| GET | ```/profiles``` | ```network_mgr::load_all_profiles``` |
|get_net_profile| GET | ```/profile/{id}``` | ```network_mgr::get_simple_profile_by_id``` |
|create_net_profile| POST | ```/profile``` | ```network_mgr::create_profile``` | NetworkManagerProfileTypeFormData |
|update_net_profile| PATCH | ```/profile``` | ```network_mgr::modify_profile``` | NetworkManagerProfileFormData |
|delete_net_profile| DELETE | ```/profile``` | ```network_mgr::delete_profile``` | NetworkManagerProfileFormData |
|get_net_interfaces| GET | ```/interfaces``` | ```network_mgr::get_all_interfaces``` |

- Structure du payload HostnameFormData

| Nom du champs | Type |
|-|-|
|hostname|String|

- Structure du payload NetworkManagerProfileTypeFormData

| Nom du champs | Type |
|-|-|
|profile_type|NetworkManagerProfileType|

- Structure du payload NetworkManagerProfileFormData

| Nom du champs | Type |
|-|-|
|profile|NetworkManagerProfile|

###### CfgMgr Scope
Ce scope expose les routes qui concernent la gestion des profils de connexion.
Le préfixe du scope NetMgr est ```/cfg_mgr``` qui donne ```/cfg_mgr/profiles``` par exemple.

| Nom du service | Méthode | Route | Fonction appelé | Structure du payload (optionnel) |
|-|-|-|-|-|
|get_conn_profiles| GET | ```/profiles``` | ```config_mgr::load_all_profiles``` |
|get_conn_profile| GET | ```/profile/{id}``` | ```config_mgr::get_profile_by_id``` |
|query_conn_profiles| GET | ```/profiles/{query}``` | ```config_mgr::get_profiles``` |
|create_conn_profile| POST | ```/profiles``` | ```config_mgr::create_profile``` |
|update_conn_profile| PATCH | ```/profiles``` | ```config_mgr::save_profile``` | ProfileFormData |
|delete_conn_profile| DELETE | ```/profile/{id}``` | ```config_mgr::delete_profile``` |

- Structure du payload ProfileFormData

| Nom du champs | Type |
|-|-|
|profile|Profile|

###### RsMgr Scope
Ce scope expose les routes qui concernent la gestion du configuration réseau.
Le préfixe du scope NetMgr est ```/rs_mgr``` qui donne ```/rs_mgr/connect``` par exemple.

| Nom du service | Méthode | Route | Fonction appelé | Structure du payload (optionnel) |
|-|-|-|-|-|
|connect| POST | ```/connect``` | ```remote_session_mgr.create_session``` | ProfileFormData |
|disconnect| POST | ```/disconnect/{id}``` | ```remote_session_mgr.disconnect_session``` |
|get_session| GET | ```/session/{id}``` | ```remote_session_mgr.get_session_by_id``` |

- Structure du payload ProfileFormData

| Nom du champs | Type |
|-|-|
|profile|Profile|

##### Tests unitaires

#### ConfigMgr
Le module ConfigMgr gère les profils de connexion de session distante avec des fonctions CRUD (Création, Lecture, Mise à Jour, Suppression). Ses fonctionnalités sont appelées depuis le Invoke Handler du WebView et donc depuis le JS de l'interface utilisateur.

{{ fig("/img/config_mgr_module.svg", " Architecture module ConfigMgr", 85) }}
##### Data flow
{{ fig("/img/ConfigMgr_data_flow.png", " ConfigMgr data flow", 85) }}
##### Fonctions

```get_profiles```: Récupère tous les profils de connexion répondant à une requête de recherche

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|query|String|Nom/Addresse à utiliser pour filtrer les profils|

<hr />

Type de retour
|Type|Description|
|-|-|
|Result<<Profiles\>, String>|Objet contentant une liste des profils ou message d'erreur|

<hr class="tableSeperator" />

```get_profile_by_id```: Récupère un profil de connexion à partir de son identifiant

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|id|String|Identifiant unique du profil demandé|

<hr />

Type de retour
|Type|Description|
|-|-|
|Option<Profile\>|Profil avec l'identifiant unique demandé s'il existe|

<hr class="tableSeperator" />

```load_all_profiles```: Instancie tous les profils depuis des enregistrements dans un fichier .toml

<hr />

Type de retour
|Type|Description|
|-|-|
|Result<Profiles, String\>|Objet contentant une liste des profils ou message d'erreur|

<hr class="tableSeperator" />

```save_profile```: Sauvegarde un profil modifié

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|profile|Profile|Profil à sauvegarder|

<hr class="tableSeperator" />

```save_profiles```: Sauvegarde tous les profils dans un fichier .toml

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|profiles|&Profiles|Référence d'objet contenant une liste de profils|

<hr class="tableSeperator" />

```create_profile```: Instancie et sauvegarde un nouveau profil

<hr />

Type de retour
|Type|Description|
|-|-|
|Result<String, String\>|Identifiant unique du profil crée ou message d'erreur|

<hr class="tableSeperator" />

```delete_profile```: Supprime un profil de connexion

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|profile|Profile|Profil à supprimer|

##### Tests unitaires

<div style="page-break-after: always;"></div>

#### NetworkMgr
Le module NetworkMgr permet de faire des appels vers NetworkManager pour configurer les interfaces réseau afin de pouvoir se connecter au réseau local et éventuellement à un VPN.

{{ fig("/img/network_mgr_module.svg", " Architecture module NetworkMgr", 85) }}
##### Data flow
{{ fig("/img/NetworkMgr_data_flow.png", " NetworkMgr data flow", 85) }}

<div style="page-break-after: always;"></div>

##### Fonctions
```exec_command```: Exécute une commande de l'outil système de configuration réseau

<hr />
Arguments

| Nom | Type | Description |
|-|-|-|
|args|Vec<&str>|Liste d'arguments à passer à l'outil système de configuration réseau|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<String, String\>|Sortie "stdout" contenant le retour de l'outil réseau ou "stderr" contentant l'erreur retourné par la commande
|Result<String, String\>|Nom d'hôte du système ou message d'erreur|

<hr class="tableSeperator"/>

```set_hostname```: Affecte le nom d'hôte de la machine locale

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<String, String\>|Nom d'hôte du système ou message d'erreur|

<hr class="tableSeperator"/>

```get_all_interfaces```: Récupère les interfaces réseau de la machine locale

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<Vec<Interface>, String\>|Liste d'interfaces réseau ou message d'erreur|

<hr class="tableSeperator" />

```get_interface_by_name```: Récupère une interface selon son nom

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|
|name|String|Nom de l'interface réseau recherché|

<hr />

Type de retour

|Type|Description|
|-|-|
|Option<Interface\>|L'interface réseau s'il existe|

<hr class="tableSeperator" />

```get_interface_address```: Récupère l'adresse IP d'une interface

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|
|interface|Interface|Interface depuis laquelle récupérer les adresses IP|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<Vec<IpAddr>, String>|Liste des adresses IP ou message d'erreur|

<hr class="tableSeperator" />

```load_all_profiles```: Charge tous les profils réseau depuis l'outil de réseau

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<Vec<NetworkManagerProfile\>, String>|Liste de profils réseau ou message d'erreur|

<hr class="tableSeperator" />

```get_simple_profile_by_id```: Récupère des informations basiques sur un profil réseau à partir de son identifiant

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|
|id|String|Identifiant unique du profil recherché|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<Vec<NetworkManagerProfile\>, String>|Liste de profils réseau ou message d'erreur|

<hr class="tableSeperator" />

```get_detailed_profile_by_id```: Récupère des informations détaillées sur un profil réseau à partir de son identifiant
Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<Vec<NetworkManagerProfile\>, String>|Liste de profils réseau ou message d'erreur|

<hr class="tableSeperator" />

```create_profile```: Crée un nouveau profil réseau avec l'outil réseau

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|
|profile_type|NetworkManagerProfileType|Type de profil réseau à créer (Wifi, Ethernet, etc.)|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<String, String\>|Identifiant unique du profil crée ou message d'erreur|

<hr class="tableSeperator" />

```modify_profile```: Modifie un profil réseau avec l'outil réseau

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|
|profile|NetworkManagerProfile|Profil avec valeurs modifiées|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<(), String>|Retour vide ou message d'erreur|

<hr class="tableSeperator" />

```delete_profile```: Supprime un profil réseau avec l'outil

<hr />

Arguments

| Nom | Type | Description |
|-|-|-|
|network_tool|&NetworkTool|Référence vers l'instance de l'outil réseau|
|profile|NetworkManagerProfile|Profil à supprimer|

<hr />

Type de retour

|Type|Description|
|-|-|
|Result<(), String>|Retour vide ou message d'erreur|


##### Tests unitaires
- ```test::get_hostname_test```: Test que la commande pour récupérer le nom d'hôte est correcte
- ```test::set_hostname_test```: Test que la commande pour affecter le nom d'hôte est correcte
- ```test::get_all_interfaces_test```: Test que la commande pour récupérer les interfaces est correcte
- ```test::get_interface_by_name_test```: Test que la récupération d'interface réussit
- ```test::get_interface_address_test```: Test que la récupération d'adresse réussit
- ```test::load_all_profiles_test```: Test que la récupération de profils réussit
- ```test::get_simple_profile_by_id_test```: Test que la récupération de profil simple réussit
- ```test::get_detailed_profile_by_id_test```: Test que la récupération de profil détaillée réussit
- ```test::create_profile_test```: Test que la commande pour créer un profil est correcte
- ```test::modify_profile_test```: Test que la commande pour modifier un profil est correcte
- ```test::delete_profile_test```: Test que la commande pour supprimer un profil est correcte
- ```test::exec_command_test```: Test que l'outil réseau puisse accepter des commandes correctement
#### RemoteSessionMgr
Le module RemoteSessionMgr lance les sessions distantes en utilisant les options de connexion soit fournies par l'utilisateur soit par un profil chargé par l'utilisateur. Ce module fait appel aux commandes telles que xfreerdp, vncviewer, Xnest ou ssh. Le sous module ```remote_protocols::xdmcp``` prends en main la négociation XDMCP qui est fait manuellement par rapport aux autres protocoles disponibles.

{{ fig("/img/remote_session_mgr_module.svg", " Architecture module RemoteSessionMgr", 85) }}
##### Data flow
{{ fig("/img/RemoteSessionMgr_data_flow.png", " RemoteSessionMgr data flow", 85) }}
##### Fonctions
- ```connect```: Se connecte à un protocole distant du profil de connexion fourni
- ```remote_protocols::open_udp_socket```: Ouvre un canal de communication UDP entre un serveur distant et la machine actuelle
- ```remote_protocols::xdmcp::send```: Envoie un packet du protocole XDMCP
- ```remote_protocols::xdmcp::recv```: Attend la réception d'un packet du protocole XDMCP
- ```remote_protocols::xdmcp::open_display```: Ouvre un écran virtuel X11
- ```remote_protocols::xdmcp::open_session```: Négocie une session XDMCP avec un serveur XDMCP distant
- ```remote_protocols::xdmcp::build_request_packet```: Construit un packet de l'opération Request du protocole XDMCP
- ```remote_protocols::xdmcp::build_manage_packet```: Construit un packet de l'opération Manage du protocole XDMCP
- ```remote_protocols::xdmcp::add_xauth_cookie```: Ajoute un cookie d'authentification MIT_MAGIC_COOKIE-1 au XAuthority du système
- ```remote_protocols::xdmcp::read_card```: Lit un nombre de bytes d'un buffer à un offset donné depuis le buffer
- ```remote_protocols::xdmcp::read_card_8```: Lit une valeur de taille 1 byte à un offset donné depuis le buffer
- ```remote_protocols::xdmcp::read_card_16```: Lit une valeur de taille 2 bytes à un offset donné depuis le buffer
- ```remote_protocols::xdmcp::read_card_32```: Lit une valeur de taille 4 bytes à un offset donné depuis le buffer
- ```remote_protocols::xdmcp::read_array_8```: Lit un array de valeurs 1 byte de taille variable à un offset donné depuis le buffer
- ```remote_protocols::xdmcp::append_card_8```: Ajoute une valeur de taille 1 byte à la fin du buffer
- ```remote_protocols::xdmcp::append_card_16```: Ajoute une valeur de taille 2 bytes à la fin du buffer
- ```remote_protocols::xdmcp::append_card_32```: Ajoute une valeur de taille 4 bytes à la fin du buffer
- ```remote_protocols::xdmcp::append_array_8```: Ajoute un array de valeurs 1 byte de taille variable à la fin du buffer
- ```remote_protocols::xdmcp::append_array_16```: Ajoute un array de valeurs 2 bytes de taille variable à la fin du buffer
- ```remote_protocols::xdmcp::append_array_of_array_8```: Ajoute un array de array de valeurs 2 bytes de taille variable à la fin du buffer
- ```remote_protocols::xdmcp::vec_u16_to_be_vec_u8```: Convertit un vecteur de valeurs de 2 bytes en vecteur de valeurs de 1 byte big-endian
- ```remote_protocols::xdmcp::vec_u8_to_string``` Convertit un vecteur de valeurs 1 byte en string hexadécimale
##### Tests unitaires

## Tests
### Tests unitaires
Rust propose des tests unitaires parallélisés intégrés dans les outils de base. L'outil en ligne de commande est ```cargo test```. De plus, les tests peuvent être étendus avec des crates prévues à cet effet comme rstest ou mockall, qui sont des crates qui proposent des tests data-driven et du mocking automatique pour des traits/structs. 

Les tests sont exécutés lors du développement sur la machine locale, ainsi que sur Github grâce à Github Actions à chacun des push vers le repo. Les tests de Github Actions sont exécutés dans un containeur sain où les étapes de setup nécessaire sont refait à chaque push pour s'assurer que le build peut être déployé et utilisé sur un système vierge et qu'il n'y a pas de problèmes d'état entre deux builds liée à la machine de test.
#### Périmètre des tests
Les scénarios suivants sont testés :

- Les paniques
- Lancement du WebView
- La génération de la page web réussit
- Que la génération de profil se crée, lit, modifie, et supprime
- Que la génération de configuration réseau se crée, lit, modifie et supprime
- Que l'envoi et la réception de packet TCP/UDP s'effectue

##### Format description des tests
Le format choisi pour décrire les tests unitaires est le suivant :
```md
###### fn_name_test
Description de la fonction.

| Propriété | Valeur |
|-|-|
| **Nom** | ```fn_name_test``` |
| **Nom de la fonction testée** | ```fn_name``` |
| **Fichier** | ```file.rs``` |
| **Cas *n*** ||
| *Description* | Description du cas testée |
| *Type de résultat attendu* | Réussite/Échec |
| *Critère(s) d'acceptation* ||
| *Critère(s) d'échec* ||

```
Ce qui donne le rendu suivant :
###### fn_name_test
Description de la fonction.

| Propriété | Valeur |
|-|-|
| **Nom** | ```fn_name_test``` |
| **Nom de la fonction testée** | ```fn_name``` |
| **Fichier** | ```file.rs``` |
| **Cas *n*** ||
| *Description* | Description du cas testée |
| *Type de résultat attendu* | Réussite/Échec |
| *Critère(s) d'acceptation* ||
| *Critère(s) d'échec* ||

##### Description des tests
###### exec_command_test
La fonction ```exec_command```, provenant du trait NetworkTool, exécute une commande shell avec les arguments fournis pour l'outil implémenté (dans ce cas ```nmcli``` de NetworkManager) et rend soit le stdout en valeur de type Ok(String) ou le stderr en valeur de type Err(String).

| Propriété | Valeur |
|-|-|
| **Nom** | ```exec_command_test``` |
| **Nom de la fonction testée** | ```exec_command``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure que la commande ```nmcli``` avec les arguments ```connection show``` rend une valeur de type Ok(String). Ceci vérifie qu'une commande valide émet une valeur Ok avec le stdout de la commande. |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(String) avec stdout comme contenu émis |
| *Critère(s) d'échec* | Valeur de type Err(String) avec stderr comme contenu émis |
| **Cas 2** ||
| *Description* | Cas qui assure que la commande ```nmcli``` avec les arguments ```show``` rend une valeur de type Ok(String). Ceci vérifie qu'une commande invalide émet une valeur Err avec le stderr de la commande. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) avec stderr comme contenu est émis |
| *Critère(s) d'échec* | Valeur de type Ok(String) avec stdout comme contenu est émis |

###### get_hostname_test
La fonction ```get_hostname``` utilise le NetworkTool fourni pour récupérer le nom d'hôte de la machine.

| Propriété | Valeur |
|-|-|
| **Nom** | ```get_hostname_test``` |
| **Nom de la fonction testée** | ```get_hostname``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la récupération du nom d'hôte de la machine réussit |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(String) contenant le nom d'hôte est émis |
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis |
| *Critère(s) d'échec* | Valeur de type Ok(String) contenant le nom d'hôte est émis |

###### set_hostname_test
La fonction ```set_hostname``` utilise le NetworkTool fourni pour affecter le nom d'hôte de la machine.

| Propriété | Valeur |
|-|-|
| **Nom** | ```set_hostname_test``` |
| **Nom de la fonction testée** | ```set_hostname``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que l'affectation de nom d'hôte réussit. |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(String) émis |
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis |
| *Critère(s) d'échec* | Valeur de type Ok(String) émis |

###### get_all_interfaces_test
La fonction ```get_all_interfaces``` utilise le NetworkTool fourni pour récupérer les interfaces réseau de la machine.

| Propriété | Valeur |
|-|-|
| **Nom** | ```get_all_interfaces_test``` |
| **Nom de la fonction testée** | ```get_all_interfaces``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la récupération de la liste d'interfaces réseau réussit |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(Vec<Interface\>) non vide émis |
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis |
| *Critère(s) d'échec* | Valeur de type Ok(Vec<Interface\>) émis |

###### get_interface_by_name_test
La fonction ```get_interface_by_name``` utilise le NetworkTool fourni pour récupérer une interface réseau de la machine depuis son nom.

| Propriété | Valeur |
|-|-|
| **Nom** | ```get_interface_by_name_test``` |
| **Nom de la fonction testée** | ```get_interface_by_name``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la récupération d'une interface mock loopback fonctionne |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Some(Interface) contenant une interface avec les propriétés renseignées est émis |
| *Critère(s) d'échec* | Valeur de type None est émis |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que dans la possibilité ou il n'y a pas d'interface assignée à un profil, la valeur None est émise |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type None est émis |
| *Critère(s) d'échec* | Valeur de type Some est émis |
| **Cas 3** ||
| *Description* | Cas qui assure avec un MockNetworkTool que si l'interface avec le nom donnée n'existe pas, la valeur None est émise |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type None est émis |
| *Critère(s) d'échec* | Valeur de type Some est émis |

###### load_all_profils_test
La fonction ```load_all_profiles``` charge et instancie tous les profils de connexion stockée par le NetworkTool fourni.

| Propriété | Valeur |
|-|-|
| **Nom** | ```load_all_profiles_test``` |
| **Nom de la fonction testée** | ```load_all_profiles``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la récupération de profil fonctionne |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(Vec<Interface\>) contenant un profil avec les propriétés renseignées est émis|
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis |
| *Critère(s) d'échec* |Valeur de type Ok(Vec<Interface\>) est émis |

###### create_profil_test
La fonction ```create_profile``` crée un profil de connexion réseau avec le NetworkTool fourni.

| Propriété | Valeur |
|-|-|
| **Nom** | ```create_profile_test``` |
| **Nom de la fonction testée** | ```create_profile``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la création de profils de connexion réseau fonctionne. |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(String) contenant l'identifiant du nouveau profil est émis |
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis |
| *Critère(s) d'échec* | Valeur de type Ok(String) est émis|

###### get_simple_profil_by_id_test
La fonction ```get_simple_profile_by_id``` récupère un profil avec des informations basiques par son identifiant unique.

| Propriété | Valeur |
|-|-|
| **Nom** | ```get_simple_profile_by_id_test``` |
| **Nom de la fonction testée** | ```get_simple_profile_by_id``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la récupération des informations basique depuis un identifiant fonctionne. |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(NetworkManagerProfil) contenant le profil demandé est émis |
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis  |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur ou que le profil demandé n'a pas été trouvé.  |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis |
| *Critère(s) d'échec* | Valeur de type Ok(NetworkManagerProfil) est émis |

###### get_detailed_profil_by_id_test
La fonction ```get_detailed_profile_by_id``` récupère un profil avec des informations détaillées par son identifiant unique.

| Propriété | Valeur |
|-|-|
| **Nom** | ```get_detailed_profile_by_id_test``` |
| **Nom de la fonction testée** | ```get_detailed_profile_by_id``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la récupération des informations détaillées depuis un identifiant fonctionne. |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(NetworkManagerProfil) contenant le profil demandé est émis |
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis |
| **Cas 2** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur ou que le profil demandé n'a pas été trouvé. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis |
| *Critère(s) d'échec* | Valeur de type Ok(NetworkManagerProfil) est émis |

###### modify_profil_test
La fonction ```modify_profile``` modifie un profil avec le NetworkTool fourni.

| Propriété | Valeur |
|-|-|
| **Nom** | ```modify_profile_test``` |
| **Nom de la fonction testée** | ```modify_profile``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas *n*** ||
| *Description* | Description du cas testée |
| *Type de résultat attendu* | Réussite/Échec |
| *Critère(s) d'acceptation* ||
| *Critère(s) d'échec* ||

###### delete_profil_test
La fonction ```delete_profile``` supprime un profil avec le NetworkTool fourni.

| Propriété | Valeur |
|-|-|
| **Nom** | ```delete_profile_test``` |
| **Nom de la fonction testée** | ```delete_profile``` |
| **Fichier** | ```network_mgr.rs``` |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la suppression d'un profil fonctionne.  |
| *Type de résultat attendu* | Réussite |
| *Critère(s) d'acceptation* | Valeur de type Ok(()) est émis |
| *Critère(s) d'échec* | Valeur de type Err(String) contenant un message d'erreur est émis |
| **Cas 1** ||
| *Description* | Cas qui assure avec un MockNetworkTool que la gestion d'erreur fonctionne. Ceci est dans le cas que la commande exécutée émet une erreur. |
| *Type de résultat attendu* | Échec |
| *Critère(s) d'acceptation* | Valeur de type Err(String) contenant un message d'erreur est émis  |
| *Critère(s) d'échec* | Valeur de type Ok(()) est émis |

### Tests de compatibilité hardware (Intégration)
Les tests d'intégration hardware servent à informer la portée possible de déploiement du programme. Rust est conçu pour être multiplateforme, mais il y a certaines dépendances qui auront besoin d'être vérifiées avant d'être sûr de la compatibilité avec les architectures système visées.
#### Procédure définie
1. Installer Blackrust et ses dépendances
2. Lancer Blackrust
3. Observer de possibles délais/lag avec l'interface WebView/WebAssembly
4. Lancer une session d'accès distant avec RDP, XDMCP et VNC
5. Observer délais/lag avec session d'accès distant

## Difficultés rencontrées
### Installation de la sous dépendance keyboard-config interrompait l'installation de dépendances dans un runner Github Actions CI
lors de l'installation du paquet ```xserver-xorg```, la dépendance de cette dernière ```keyboard-config``` demande un saisi utilisateur qui ne peut pas être effectué dans l'exécution automatique du script de test. Donc la solution pour cela est d'exporter une variable d'environnement lors de l'installation des dépendances ```sudo DEBIAN_FRONTEND=noninteractive apt-get -y install ...```. Ceci force keyboard-config à prendre une valeur par défaut et laisser le reste de l'installation se poursuivre.
### Tests unitaires utilisant le serveur d'affichage ne réussissent pas sur Github Actions CI
Pour certains tests unitaires, un serveur d'affichage X.Org est nécessaire, mais cela n'est pas installé dans les containeurs de runner Github Actions CI. Donc la solution pour cela est d'installer les paquets ```xserver-xorg``` et ```xserver-xorg-video-dummy```. Ceci permet de faire un serveur X11 en mode headless afin de valider que le programme s'exécute et affiche l'interface WebView. Afin de spécifier le fait que nous voulons utiliser le driver ```xserver-xorg-video-dummy```, nous devons créer un fichier de configuration comme la suivante et exécuter ```X :0 -config .github/workflows/xorg-dummy.conf &``` à la racine du projet :

```conf
# Source: https://techoverflow.net/2019/02/23/how-to-run-x-server-using-xserver-xorg-video-dummy-driver-on-ubuntu/
Section "Monitor"
  Identifier "Monitor0"
  HorizSync 28.0-80.0
  VertRefresh 48.0-75.0
  # https://arachnoid.com/modelines/
  # 1920x1080 @ 60.00 Hz (GTF) hsync: 67.08 kHz; pclk: 172.80 MHz
  Modeline "1920x1080_60.00" 172.80 1920 2040 2248 2576 1080 1081 1084 1118 -HSync +Vsync
EndSection

Section "Device"
  Identifier "Card0"
  Driver "dummy"
  VideoRam 256000
EndSection

Section "Screen"
  DefaultDepth 24
  Identifier "Screen0"
  Device "Card0"
  Monitor "Monitor0"
  SubSection "Display"
    Depth 24
    Modes "1920x1080_60.00"
  EndSubSection
EndSection
```
### Échec du test unitaire open_webview_test causé par la récupération de nom d'hôte
Lors du test unitaire open_webview_test qui vérifie que le WebView peut être construit et affiché dans le serveur d'affichage, la récupération du nom d'hôte provoque un SIGABRT (process abort signal) alors que le test s'est bien effectué. Ceci est le cas, car, le test ferme l'application juste après que l'appel est fait et en conséquence fait une erreur quand il ne peut pas exécuter le code JS permettant d'afficher le résultat. La solution à ce problème et de déplacer l'appel vers network_mgr pour récupérer le nom d'hôte dans le invoke "init" du WebView afin de retarder l'appel et ne pas provoquer d'appels qui ne pourront pas être aboutis.

## Livrables
- Documentation
    - Cahier des charges
    - Journal de bord
    - Documentation technique
    - Manuel utilisateur
- Programme
    - Paquet avec scripts d'installation (PKGBUILD)
    - Code source ([Github](https://github.com/DylanUpchr/Blackrust))
## Améliorations possibles
### Tests unitaires / fonctionnels
Plus de tests unitaires, surtout sur la construction de l'API Actix ou l'application Yew, ainsi que des tests fonctionnels sur l'interface Yew.


## Conclusion
En conclusion, j'ai développé un programme Rust pour Linux, qui permet de se connecter à plusieurs types de VDI ou serveur distant à travers des connexions sécurisées.

L'application peut utiliser plusieurs protocoles d'accès distant tels que le RDP, VNC et XDMCP et est déployable sur de différentes architectures telles que ARMv8 ou x86_64 sur Linux.
## Bilan personnel
En fin de compte, j'ai pu faire un outil que j'utilise quotidiennement afin de me connecter à mes différentes machines virtuelles sur mon réseau local depuis chez moi ainsi qu'à distance. 

Faire un programme de cette ampleur en Rust m'as vraiment plu, car j'ai pris beaucoup de plaisir à approfondir mes connaissances dans ce langage et de découvrir davantage de technologies, telles que le WebAssembly. J'ai également pu approfondir mes connaissances des différents protocoles d'accès distant utilisés comme le RDP, le XDMCP et le VNC.

## Tableau des figures
{{ figListing() }}

## Glossaire
{{ lexListing() }}