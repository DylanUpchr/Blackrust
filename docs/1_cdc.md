# Cahier des charges
## Résumé / Abstract
Blackrust est un logiciel multiarchitecture pour Linux qui, au lancement de la machine, propose des sessions d'accès distant sécurisées utilisant de divers protocoles ainsi qu'une session locale hors-ligne.

Ce projet est un client léger qui a pour but de réduire la taille et le coût de moult machines données aux employés dans une entreprise avec de l'infrastructure VDI (virtualisation du poste de travail). Ces clients légers se connecteront à un serveur central où résideront les espaces de travail des utilisateurs avec davantage de puissance de calcul que sur la machine locale.

---

Blackrust is a multi-architecture program for Linux that, at the startup of the computer, offers a secure remote desktop session via many protocols as well as an offline local desktop.

This project is a thin client, which aims to reduce the size and cost of the many machines given to employees in a company using VDI (virtual desktop infrastructure). These thin clients will connect to a centralized server where the user's workspace resides and offer greater processing power than the local machine.
## Objectifs
Logiciel qui permet de remplir les objectifs suivants:

- Démarrer une session d'accès distant avec un des protocoles suivants|
    - RDP
    - VNC
    - XDMCP
    - SSH X11-Forwarding
- Démarrer un desktop en local
- Démarrer un serveur audio distant dans le cas d'une session XDMCP/X11-Forwarding
- Configurer le réseau local à travers NetworkManager
- Configurer une connexion VPN à travers NetworkManager
- Sauvegarder / Modifier / Charger des profils de connexion
- Sauvegarder / Modifier / Charger des thèmes
## Parties prenantes
| Nom | Fonction |
|-|-|
| Yannick Zeltner | Enseignant de suivie / Mandant |
| Dylan Upchurch | Élève/ Développeur / Mandataire |

## Environnement
L'environnement de travail utilisé lors du développement de ce projet consiste en :
#### Matériel
- Ordinateur de l'école "upchr-arch"
- Raspberry Pi Model 4B (4GB) "Testbed-Rpi"
- Jetson Nano Developer Kit "Testbed-JN"
#### Software
##### Arch Linux (upchr-arch)
- Visual Studio Code
- Rust "Stable"

#### Raspbian / Debian 11 Bullseye (Testbed-Rpi)
- Blackrust
#### Arch Linux (Testbed-Rpi)
- Blackrust
#### Linux4Tegra (Testbed-JN)
- Blackrust

## Technologies utilisées
| Type | Nom |
|-|-|
| Langage de programmation backend | Rust |
| Langages de programmation frontend | HTML |
| | CSS |
| | JS |
| Protocoles Accès distants | RDP |
| | VNC |
| | SSH X11 Forwarding |
| | XDMCP |
| Serveur d'affichage | X11 |
| Server audio | PulseAudio |
| Outil réseau (LAN/VPN) | NetworkManager |

## Sécurité
De base, le langage Rust est assez sécurisé grâce aux faits suivants :
- Rust est "memory-safe", qui signifie qu'il ne permet pas d'avoir des pointeurs null ou invalide
- Les courses de données sont également impossibles, grâce au système de "appartenance", qui impose qu'une instance ou référence variable ne puisse être utilisée que par une fonction à la fois.
- La gestion d'erreur est très avancée et devrait être au cœur de la conception d'une fonction. Cette approche permet d'être toujours certain que le déroulement se passe comme prévu et les cas de bords qui pourraient compromettre la sécurité de l'application sont évités.

Ceci dit, les mesures suivantes devraient être prises lors du développement :
- Aucune donnée sensible stockée, ce risque ne peut être différé à des keyrings et des gestionnaires de mot de passe tierces
- Accès au système ne doit pas dépasser celles d'un utilisateur quelconque, donc à part des vulnérabilités d'escalade de privilège externe existantes, ce risque est évité.

## Exemples d'utilisation
### Dans un cadre personnel :
Un cas d'utilisation personnel serait si l'on veut avoir un environnement de développement spécifique que l'on héberge à la maison sur un ordinateur puissant et puis on veut utiliser cet environnement à l'extérieur sur un ordinateur portable bas de gamme. L'environnement de développement aura un serveur d'accès distant tel RDP ou VNC, et possiblement par le biais d'un VPN selon notre configuration. Le logiciel Blackrust pourra proposer de se connecter à cet environnement au démarrage de l'ordinateur portable qu'on soit en vacances ou dans le jardin chez nous.

---
### Dans un cadre professionnel :
Un cas d'utilisation professionnel serait si notre employeur nous met à disposition une infrastructure de bureau virtuel avec le protocole RDP derrière un VPN, puis nous fournit un ordinateur de petite taille ou un ordinateur portable avec le logiciel Blackrust installé. Le logiciel permet de garder le même espace de travail, quel que soit l'endroit où on se trouve dans le monde, qu'on soit au bureau ou en télétravail.

## Expérience utilisateur
### Arrivée dans l'application
Blackrust v0 est conçu pour être un Display Manager Linux avec des fonctionnalités d'accès distant, du coup le programme sera un service au démarrage. Ceci entend que le programme sera la première application que l'utilisateur verra, et lui permettra de choisir une session locale ou une session sur un serveur distant.

### Configuration de l'application
Au besoin, l'utilisateur pourra adapter les réglages réseau à sa configuration, par exemple mettre une adresse fixe, changer la passerelle par défaut ou se connecter à un VPN. 

### Connexion session distante
L'utilisateur pourra soit faire une "connexion rapide", c'est-à-dire renseigner le protocole, l'IP/nom d'hôte et les options de connexion pour une connexion unique qui ne sera pas gardée, ou renseigner ces champs dans un profil sauvegardé pour que l'on puisse se reconnecter à nouveau avec aisance.

### Connexion session locale
L'utilisateur pourra également ouvrir un bureau local, au cas où il a besoin de travailler en hors-ligne ou affecter quelque chose sur la machine locale.

## Hardware
Le langage Rust permet de facilement compiler pour plusieurs architectures cibles, donc ce logiciel pourrait être déployé sur des systèmes tel ARMv7, ARMv8 ou x86_64. Il reste à voir si toutes les dépendances du projet seraient satisfaites sous ces différentes architectures et sous différentes distributions Linux. Une partie du travail de diplôme serait une analyse de compatibilité et de faire des benchmarks avec différentes combinaisons hardware (Raspberry Pi 4, Jetson Nano, etc.) et distribution Linux (Debian, Arch) sous différentes architectures (ARMv7, ARMv8 et x86_64).

## Analyse concurrentielle
### [Remmina](https://remmina.org/)
Remmina est un client de desktop remote pour linux écrit en C et qui utilise la librairie GTK+ pour se connecter à plusieurs types de sessions distantes telles que  SSH, VNC, RDP, NX, XDMCP, et même des interfaces HTTP/HTTPS qu'on retrouve sur des routeurs.

Ce logiciel remplit le même besoin et on ressemble beaucoup à ce projet, mais il y a une différence principale entre les deux. Blackrust sera disponible dès le lancement du client, et prendra directement en charge le lancement de la session distant ou de la session locale selon le choix de l'utilisateur. Remmina est une application GTK+ qui est lancée sur le bureau donc intrinsèquement utilise plus de ressources que Blackrust.

Ce programme est gratuit, open source et sous la licence Copyleft.

#### Avantages
- Remmina propose les protocoles NX et HTTP/HTTPS en plus de celles de Blackrust
- Système modulaire de plugins pour les protocoles distants
- Interface simple, mais fonctionnelle
- Open source

### Inconvénients
- Doit être lancé depuis le bureau Linux

### [MobaXterm](https://mobaxterm.mobatek.net/)
MobaXTerm est un client d'accès distant (SSH, telnet, rlogin, Mosh, RDP, VNC et XDMCP), terminal avec serveur Xorg intégré, et une compilation d'outils système (CygUtils liste matérielle/processus ainsi qu'un package manager) et réseau (Serveurs HTTP/telnet/FTP/NFS/VNC/Cron, tunnels SSH, SSH Keygen, netstat, WakeOnLAN, nmap, CygUtils packet capture).

En plus de tout cela, il propose quelques jeux simples et un éditeur de texte. MobaXterm est un environnement de bureau Linux complet sur Windows et va bien au-delà de la portée de l'accès distant sur la quel Blackrust se concentre.

Il existe une version gratuite pour l'utilisation personnelle ainsi qu'une version payante avec davantage de fonctionnalités pour les utilisateurs professionnels.

Ce programme est propriétaire et distribué sous des licences EULA propres à l'entreprise qui l'a développé, Mobatek.

#### Avantages
- Environnement de bureau Linux complet (accès distant, outils, jeux) sur Windows 
- Système modulaire de plugins pour les outils
- Nombre d'outils convenables pour un utilisateur expérimenté / développeur
- Propriétaire

#### Inconvénients
- Doit être lancé depuis le bureau Windows
- Interface complexe
- Nombre d'outils imposant et possiblement intimidant pour l'utilisateur moyen.

### [ThinLinc](https://www.cendio.com/)
ThinLinc est un environnement d'accès distant complet basé sur le VNC qui utilise l'authentification par tunnel SSH avec les entreprises comme public cible. 

Ils proposent une solution propriétaire qui utilise des librairies open source, afin d'avoir un système client/serveur pour les clients légers internes, et même du télétravail selon la configuration réseau. Ils font partie de la concurrence à ce projet, car ils se situent dans le même domaine et remplissent le même cas d'utilisation, mais ils proposent un écosystème d'accès distant complet alors que Blackrust est un client polyvalent pour les installations basiques de RDP/VNC/SSH/XDMCP.

### [Citrix](https://www.citrix.com/)
Citrix est un environnement d'accès distant complet basé sur le RDP/RDS qui propose de meilleures performances que le RDP/RDS basique avec les entreprises comme public cible. 

Ils proposent une solution propriétaire qui utilise des librairies open source, afin d'avoir un système client/serveur pour les clients légers internes, et même du télétravail selon la configuration réseau. Ils font partie de la concurrence à ce projet, car ils se situent dans le même domaine et remplissent le même cas d'utilisation, mais ils proposent un écosystème d'accès distant complet alors que Blackrust est un client polyvalent pour les installations basiques de RDP/VNC/SSH/XDMCP.
## Analyse système
![Analyse système](./img/blackrust-systems-analysis.svg)
Le programme est décomposé en 5 modules principaux :

#### Main (DM): Point d'entrée du programme et aperçu graphique
#### ConfigMgr: CRUD pour les options de connexion sauvegardées
#### NetworkMgr: Module qui configure le réseau (IPv4, IPv6, configuration VPN) à travers la commande `nmcli` de [NetworkManager](https://networkmanager.dev/)
#### RemoteSessionMgr: Lanceur de sessions distant
##### RemoteProtocols 
- XDMCP
- VNC
- RDP
- SSH
#### BlackrustLib: Fonctions communes à plusieurs modules, librairie interne

## Analyse heuristique
![Analyse heuristique](./img/blackrust-mind-map.svg)

## Livrables
### Documentation
- Cahier des charges
- Journal de bord
- Rapport
- Manuel utilisateur
### Programme
- Paquet avec scripts d'installation (PKGBUILD)
- Code source ([Github](https://github.com/DylanUpchr/Blackrust))

## Glossaire
#### Serveur d'affichage
Un serveur d'affichage est un programme qui peut exploiter les ressources de rendu graphique pour afficher des applications graphiques.

#### Authentification PAM
PAM, ou Pluggable Authentification Modules, est un mécanisme dans les systèmes d'exploitation UNIX et UNIX-like qui propose des APIs pour l'authentification de bas niveau.

Linux PAM, évolué de l’UNIX PAM, propose l'authentification de compte locale, LDAP ou de lecteurs d'empreinte digitale.

#### Window Manager
Un Window Manager est un programme qui affiche des applications graphiques dans des fenêtres et gère leur disposition.

#### Display Manager
Un Display Manager est un programme qui, après le lancement du serveur d'affichage (tel X11 ou Wayland), se charge de l'authentification PAM et de lancer un Window Manager.