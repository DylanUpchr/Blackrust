# Cahier des charges
## Résumé / Abstract
Blackrust v0 est un logiciel multi-architecture pour linux qui, au lancement de la machine, proposera des sessions d'accès distant par de divers protocoles ainsi qu'une session locale hors-ligne.

---

Blackrust v0 is a multi-architecture program for linux that , at the startup of the computer, will offer a remote desktop session via many protocols aswell as an offline local desktop.


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
| TBD | Enseignant de suivie / Mandant |
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

#### Avantages
- Remmina propose les protocoles NX et HTTP/HTTPS en plus de celles de Blackrust

#### Inconvenients
- Doit être lancé depuis le bureau linux


### [MobaXterm](https://mobaxterm.mobatek.net/)

MobaXTerm est un client SSH, 
### [ThinLinc](https://www.cendio.com/)

## Glossaire
#### Display Manager

definition
#### Window Manager

definition