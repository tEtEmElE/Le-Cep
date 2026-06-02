# Le CEP

Le **CEP** est une application web développée en Rust dont l’objectif principal est de **gagner en visibilité** et de servir de vitrine pour l'église du CEP.

Le projet met l’accent sur la **simplicité**, la **performance** et une **stack moderne** côté backend comme frontend.

---

## 🎯 Objectifs

- Se faire connaître et améliorer la visibilité du projet  
- Proposer une interface web claire et accessible  
- Servir de base évolutive pour de futures fonctionnalités

---

## 🛠️ Technologies utilisées

### Backend
- **Axum** : framework web asynchrone basé sur Tokio
- **Askama** : moteur de templates Rust, rapide et sûr (compilé)

### Frontend
- **Pico.css** : framework CSS minimaliste et léger, sans JavaScript

## 🚀 Lancement du projet

### Prérequis
- Rust (édition stable)
- Cargo

### Démarrage en local
Créer le ficier default_user.json et suivre l'example de default_user_example.json en changeant les valeur.
Cela créera le user 
```bash
cp default_user_example.json default_user.json
cargo run


## TODO 
- Type de fichier déclaratif pour créer, modifier et supprimer des pages rappidement (un dossier avec l'arborésance du site -> routage automatique création des fichier html avec les bonne propriété bon titre ...)
- Port en dehors du code 