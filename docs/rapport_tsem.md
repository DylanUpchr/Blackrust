# Rapport du Travail de semestre
## Résumé / Abstract
Blackrust v0 est un logiciel multi-architecture pour linux qui, au lancement de la machine, proposera des sessions d'accès distant utilisant divers protocoles ainsi qu'une session locale hors-ligne.

Ce projet est un client léger qui a pour but de réduire la taille et le coût des moultes machines données aux employés dans une entreprise. Ces clients légers se connecteraient à un serveur central où résideraient les espaces de travail des utilisateurs avec d'avantage de puissance de calcul.

---

Blackrust v0 is a multi-architecture program for linux that, at the startup of the computer, will offer a remote desktop session via many protocols aswell as an offline local desktop.

This project is a thin client, which aims to reduce the size and cost of the many machines given to employees in a company. These thin clients will connect to a centralized server where the users workspace will be and will offer greater processing power.

## Analyse de l'existant
analyse concurencielle, parler de valeur ajoutée (multi plateforme, OSS vitesse, sécurité)
## Cahier des charges
[Lien vers le cahier des charges](index.md)
## Analyse fonctionelle
L'analyse fonctionnelle contient les maquettes, l'architecture du programme et les diagrammes explicitant son fonctionnement
### Architecture
#### Modules internes
![Analyse système](./img/blackrust-systems-analysis.svg)
Le programme est décomposé en 5 modules principaux:

- Main (DM): Point d'entrée du programme et aperçu graphique
- ConfigMgr: CRUD pour les options de connexion sauvegardées
- NetworkMgr: Module qui configure le réseau (IPv4, IPv6, configuration VPN) à travers la commande `nmcli` de [NetworkManager](https://networkmanager.dev/)
- RemoteSessionMgr: Lanceur de sessions distant
    - RemoteProtocols 
        - XDMCP
        - VNC
        - RDP
        - SSH
- BlackrustLib: Fonctions commun à plusieurs modules, librairie interne
##### Main
Le module main est le point d'entrée principale de l'application, lance l'aperçu WebView qui permet d'interfacer avec l'application et appeler les autres modules
##### ConfigMgr
Le module ConfigMgr gère les profils de connexion de session distante avec des fonctions CRUD (Création, Lecture, Mise à Jour, Suppression). Ses fonctionnalités sont appelées depuis le Invoke Handler du WebView et donc depuis le JS de l'interface utilisateur.
##### NetworkMgr
Le module NetworkMgr permet de faire des appels vers NetworkManager pour configurer les interfaces réseau afin de pouvoir se connecter au réseau local et éventuellement à un VPN.
##### RemoteSessionMgr
Le module RemoteSessionMgr lance les sessions distantes en utilisant les options de connexion soit fournies par l'utilisateur soit par un profile chargé par l'utilisateur. Ce module fait appel aux commandes tel xfreerdp, vncviewer, Xnest ou ssh.
##### Blackrust-Lib
Blackrust-Lib est la libraire commune aux modules et contient les définitions de structures de données et les fonctions utilisées par tous les modules.
#### Librairies externes
Le programme utilise également quelques libraires externes, principalement pour le rendu graphique Web.
##### Web-view
Web-view est un crate qui agit en tant que navigateur web qui affiche le rendu HTML/CSS/JS.
##### Xrandr
Xrandr permet de récupérer des informations sur le ou les écrans d'affichage, comme taille, DPI, disposition des moniteurs, etc.
##### Serde / Serde-JSON
Serde implémente des fonctionnalités de serialisation et déserialisation des instances d'objets vers et depuis le JavaScript Object Notation (JSON).
##### Image-base64
Image-base64 est un crate qui encode ou "traduit" des fichiers image en texte base64. **Ceci est nécessaire pour l'instant à cause de WebView qui ne peut pas référencer des fichiers et que traiter du HTML pur. Ceci pourra changer en implémentant Actix (Serveur Web) et Yew (Framework WASM pour Rust)**
##### Regex
Le crate Regex implémente des expressions régulières utilisées pour la vérification des données saisies par l'utilisateur pour la configuration réseau
### Maquettes

## Sécurité
### Failles possibles

## Analyse organique
### Choix du langage
J'ai choisi Rust comme langage pour le travail de semestre car c'est une langage moderne. Rust est connu pour son fiabilité, sécurité et rapidité.

#### Rapidité
Rust est connu pour sa rapidité grâce à certains caractéristiques:
- Rust est statiquement typée, donc aprés la vérification de cargo check, pleins de vérifications au runtime peuvent être sautées
- Rust n'as pas de Garbage Collector, la mémoire est alloué et libéré selon "l'éspérance de vie" d'une variable et donc ces derniers n'existent aussi longtemps que nécessaire. Ceci réduit les ressources consommées par un Garbage Collector et enlève les tâches répetitives de gérance de mémoire manuelle
- Rust utilise le LLVM pour générer du code assembly optimisé, qui est comparable au GCC en termes de performances du programme finale
#### Compilateur
L'outil de compilation de Rust, nommée cargo, a plusieurs rôles:
- Package manager, pour les "crates" qui sont les paquets/modules officiels et de la communauté
- Validateur du code, cargo check vérifie plusieurs aspects avant de compiler le programme:
    - Que la gérance du mémoire est bien fait et ne viole pas les régles d'appartenance ou d'emprunt de réferences
    - Que les variables sont nommées en snake case, sinon il affiche des warnings
    - Qu'il n'y a pas du code "mort", donc pas utilisé, sinon il affiche des warnings
- Compilateur, bien entendu si le code ne contient pas d'erreur de syntaxe, ni de gérance de mémoire le programme est compilée et rends un éxécutable dans le dossier target

Les messages d'erreurs de cargo sont assez riches comparés aux autres langages. Cargo peut décrire l'erreur détectée en détail et même selon le type d'erreur il peut suggérer des solutions. Si cela ne suffit pas, le traçage du pile d'appels est accésible et peut aider avec le débogage traditionnelle.
#### Sécurité / Fiabilité
De base, le langage Rust est assez sécure et fiable grâce au faites suivantes:
- Rust est "memory-safe", qui signifie qu'il ne permet pas d'avoir des pointeurs null ou invalide
- Les courses de données sont également impossible, grâce au système de "appartenance", qui impose qu'une instance ou réference variable ne peut être utilisé par une fonction à la fois.
- La gestion d'erreur est très avancé et devrait être au coeur de la conception d'une fonction. Cette approche permet d'être toujours certain que le déroulement se passe comme prévu et les cas de bords qui pourraient compromettre la sécurité de l'application sont évités.
- Fonctionnalités de tests unitaires intégrées

#### Multi-plateforme
Rust est une lanque avec un compilateur portable comme le langage C, donc qui peut être compilé sur la plupart des plateformes avec certains garanties de fonctionnalité. Rust catégorise ces garanties dans un système de tiers. Les tiers sont ainsi:

- Tier 1: Garantie d'éxécution, un programme en Rust pure est capable de compiler et de s'éxécuter sans problèmes
    - Exemples: x86_64 Windows, x86_64 MacOS, x86_64 Linux, AArch64 Linux (ARM64)
- Tier 2: Garantie de compiler, un programme en Rust pure est capable d'être compilé mais n'as pas une garantie 100% de fonctionner parfaitement lors de l'éxécution
    - Exemples: iOS, Android, RISC-V, MIPS/MIPS64, PowerPC/PowerPC64 
- Tier 3: Pas de garanties de compilation ni d'éxécution, mais ont une possibilité de fonctionner et pour certains des programmes on déja été faites
    - Exemples: Apple tvOS, Nintendo 3DS, CUDA, EFI

### Normes
#### Nommage
##### Rust
##### JS/HTML
#### Commentaires
#### Commits
### Organisation

### Environnement de travail
L'environnement de travail utilisé lors du développement de ce projet consistes-en:

- Ordinateur de l'école avec Arch Linux installé dessus,
- Visual Studio Code comme IDE
- Raspberry Pi 4 8G

## Difficultés rencontrées
- Appréhension de Rust
- WebView (Inclusion du CSS/JS et images encodées en base64)
- Définition des dépendances clés du projet à installer
- Compilation Multi-plateforme
## Difficultés à venir
- Design/UX
- Remplacement des technologies front-end (Passer de Webview à WebAssembly)
- Définition des cas de testes unitaires pour les différents modules
## Tests
### Tests de compatabilité hardware (Integration)
#### Procédure définit
1. Installer Blackrust et ses dépendances
2. Lancer Blackrust
3. Observer délais/lag avec l'interface WebView/WebAssembly
4. Lancer une session d'accès distant avec RDP, XDMCP et VNC
5. Observer délais/lag avec session d'accès distant
## Planning
### Prévisionnel

## Livrables
- Documentation
    - Cahier des charges
    - Journal de bord
    - Documentation technique
- Programme
    - Code source ([Github](https://github.com/DylanUpchr/Blackrust))
## Conclusion
## Bilan Personnel