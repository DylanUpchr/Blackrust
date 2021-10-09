# Cahier des charges
## Résumé / Abstract
Blackrust v0 est un logiciel multi-architecture pour linux qui, au lancement de la machine, proposera des sessions d'accès distant utilisant divers protocoles ainsi qu'une session locale hors-ligne.

Ce projet est un client léger qui a comme but de réduire la taille et le coût des moultes machines données aux employés dans une entreprise. Ces clients légers se connecteraient à un serveur central où résiderait les espaces de travail des utilisateurs avec d'avantage de puissance de calcul.

---

Blackrust v0 is a multi-architecture program for linux that , at the startup of the computer, will offer a remote desktop session via many protocols aswell as an offline local desktop.

This projet is a thin client, which aims to reduce the size and cost of the many machines given to employees in a company. These thin clients will connect to a centralized server where the users workspace will be and will offer greater processing power.
## Objectifs
Logiciel qui permet de:

- Démarrer une session d'accès distant avec un des protocoles suivants|
    - RDP
    - VNC
    - XDMCP
    - SSH X11-Forwarding
- Démarrer un desktop en local
- Démarrer un serveur audio distant dans le cas d'une session XDMCP/X11-Forwarding
- Configurer le réseau local à travers NetworkManager
- Configurer une connexion VPN à travers NetworkManager
- Sauvegarder / Modifier / Charger des profiles de connexion
- Sauvegarder / Modifier / Charger des thèmes
## Parties prenantes
| Nom | Fonction |
|-|-|
| Francisco Garcia | Enseignant de suivie / Mandant |
| Pascal Bonvin | Enseignant de suivie / Mandant |
| Dylan Upchurch | Elève / Développeur / Mandataire |

## Environnement
- Matériel
    - Ordinateur de l'école
    - Raspberry Pi Model 4B (4GB) "Testbed"
- Software
    - Arch Linux
        - Visual Studio Code
        - Rust
## Technologies utilisées
| Type | Nom |
|-|-|
| Langage de programmation backend | Rust |
| Langages de programmation frontend | HTML |
| | CSS |
| | JS |
| Protocoles Accès distant | RDP |
| | VNC |
| | SSH X11 Forwarding |
| | XDMCP |
| Serveur d'affichage | X11 |
| Server audio | PulseAudio |
| Outil réseau (LAN/VPN) | NetworkManager |

## Analyse concurrencielle
### [Remmina](https://remmina.org/)
Remmina est un client de desktop remote pour linux écrit en C et qui utilise la librairie GTK+ pour se connecter à plusieurs types de sessions distants tels que  SSH, VNC, RDP, NX, XDMCP et même des interfaces HTTP/HTTPS qu'on retrouve sur des routeurs.

Ce logiciel rempli le même besoin et on ressemble beaucoup à ce projet mais il y a une différence principale entre les deux. Blackrust sera disponible dès le lancement du client, et prendera directement en charge le lancment du session distant ou du session locale selon le choix de l'utilisateur. Remmina est une application GTK+ qui est lancé sur le bureau donc intrinsiquement utilise plus de ressources que Blackrust.

Ce programme est gratuit, open source et sous la license Copyleft.

#### Avantages
- Remmina propose les protocoles NX et HTTP/HTTPS en plus de celles de Blackrust
- Système modulaire de plugins pour les protocoles distant
- Interface simple mais fonctionnel
- Open source

#### Inconvenients
- Doit être lancé depuis le bureau Linux

### [MobaXterm](https://mobaxterm.mobatek.net/)
MobaXTerm est un client d'accès distant (SSH, telnet, rlogin, Mosh, RDP, VNC et XDMCP), terminal avec serveur Xorg integré, et une compilation d'outils sytème (CygUtils liste materiel/processus ainsi qu'un package manager) et réseau (Serveurs HTTP/telnet/FTP/NFS/VNC/Cron, tunnels SSH, SSH Keygen, netstat, WakeOnLAN, nmap, CygUtils packet capture).

En plus de tout cela il propose quelques jeux simples et un éditeur de texte. MobaXterm est un environnement de bureau linux complet sur Windows et va bien au-delà de la portée de l'accès distant sur laquel Blackrust se concentre.

Il existe une versions gratuite pour l'utilisation personnelle ainsi qu'une version payante avec d'avantage de fonctionnalités pour les utilisateurs professionelles

Ce programme est propriétaire et distribué sous des licenses EULA propres à l'entreprise qui l'a développé, Mobatek.

#### Avantages
- Environnement de bureau linux complet (accès distant, outils, jeux) sur Windows 
- Système modulaire de plugins pour les outils
- Nombre d'outils convenable pour un utilisateur experimenté / développeur
- Propriétaire

#### Inconvenients
- Doit être lancé depuis le bureau Windows
- Interface complexe
- Nombre d'outils imposant et possiblement intimidant pour l'utilisateur moyen.

### [ThinLinc](https://www.cendio.com/)


### [Citrix](https://www.citrix.com/)

## Analyse système
![Analyse système](./img/blackrust-systems-analysis.png)
## Analyse heuristique
![Analyse heuristique](./img/blackrust-mind-map.png)
## Glossaire
#### Serveur d'affichage
Un serveur d'affichage est un programme qui peut exploiter les ressources de rendu graphique pour afficher des applications graphiques.

#### Authentification PAM
PAM, ou Pluggable Authentication Modules, est un mechanisme dans les systèmes d'exploitation UNIX et UNIX-like qui propose des APIs pour l'authentification de bas niveau.

Linux PAM, évolué du UNIX PAM, propose l'authentification de compte locale, LDAP ou de lecteurs d'empreinte digitale.

#### Window Manager
Un Window Manager est un programme qui affiche des applications graphiques dans des fenêtres et gère leur disposition.

#### Display Manager
Un Display Manager est un programme qui, après le lancement du serveur d'affichage (tel X11 ou Wayland), se charge de l'authenification PAM et de lancer un Window Manager