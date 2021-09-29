# Cahier des charges
## Introduction FR/EN
Blackrust v0 est un prototype de logiciel pour linux qui, au lancement de la machine, proposera des sessions d'accès distant par de divers protocoles ainsi qu'une session locale hors-ligne.

Ce logiciel est un "Display Manager ou DM" qui démarre après le serveur d'affichage et propose de lancer un "Window Manager ou WM" qui representent le Desktop. Dans le cadre de ce logiciel le WM sera soit une session distant soit un instance de WM locale.

---

Blackrust v0 is a software prototype for linux that , at the startup of the computer, will offer a remote desktop session via many protocols aswell as an offline local desktop.

This program is a "Display Manager or DM" that starts after the display server starts and offers to start a "Window Manager or WM" that is the desktop. In the context of this program the VM will either be a remote desktop session or a local desktop.
## Objectifs
Logiciel qui permet de:

- Démarrer une session d'accès distant avec un des protocoles suivants:
    - RDP
    - VNC
    - XDMCP
    - SSH X11-Forwarding
- Démarrer un desktop en local
- Démarrer un serveur audio distant dans le cas d'une session XDMCP/X11-Forwarding
- Configurer le réseau local à travers NetworkManager
- Configurer une connexion VPN à travers NetworkManager
- 
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
| Langue de programmation | Rust |
| Protocoles Accès distant | RDP |
| | VNC |
| | SSH X11 Forwarding |
| | XDMCP |
| Serveur d'affichage | X11 |
| Server audio | PulseAudio |

## Planification
### Dates importantes / Jalons
### Dotation horaire
### Planning prévisionnel