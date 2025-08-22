# mini\_rust\_sgbd

Un mini SGBD écrit en **Rust**, avec un moteur en mémoire et un parseur maison.
Projet pédagogique pour comprendre la conception d’un SGBD, de l’AST à l’exécution de requêtes.

---

## Objectifs

* Implémenter un parseur SQL simple (SELECT, INSERT, CREATE).
* Comprendre la structure interne d’une base : Table, Schema, Row.
* Créer un moteur d’exécution minimaliste mais fonctionnel.
* Apprendre Rust via un projet structuré, avec tests unitaires et bonne architecture.

---

## Fonctionnalités prévues

* CREATE TABLE avec schéma (types `Int` et `Text`).
* INSERT de lignes dans les tables.
* SELECT simple avec projection `*` et éventuellement filtrage `WHERE`.
* Gestion des erreurs lexicales, syntaxiques et sémantiques.
* Stockage optionnel en JSON ou binaire pour persistance.
* Interface REPL simple pour entrer des requêtes.

---

## Architecture

```
src/
 ├── core/       # types, erreurs, invariants
 ├── frontend/   # lexer, tokens, parser, AST
 ├── executor/   # moteur, évaluation d’expressions
 ├── storage/    # sérialisation JSON/Bincode
 ├── interface/  # REPL ou CLI
 ├── main.rs     # point d’entrée
 └── lib.rs      # export des modules
```

* **Core** : défend les invariants, centralise les types et erreurs.
* **Frontend** : transforme du texte en AST.
* **Executor** : applique l’AST sur les données en mémoire.
* **Storage** : persiste les tables.
* **Interface** : interface utilisateur simple pour exécuter les requêtes.

---

## Installation

1. Installer [Rust](https://www.rust-lang.org/tools/install).
2. Installer [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (normalement inclus avec Rust).
3. Cloner le projet :

```bash
git clone <URL_DU_REPO>
cd mini_sql
```

---

## Compilation et exécution

* Compiler :

```bash
cargo build
```

* Lancer REPL :

```bash
cargo run
```

* Exécuter les tests :

```bash
cargo test
```

* Vérifier le formatage :

```bash
cargo fmt -- --check
```

* Vérifier les lints :

```bash
cargo clippy
```

---

## Exemple d’utilisation (REPL)

```
sql> CREATE TABLE users (id Int, name Text);
sql> INSERT INTO users VALUES (1, 'Alice');
sql> SELECT * FROM users;
Output:
id | name
1  | Alice
```

---

## Conventions

* **Nom des modules** : snake\_case.
* **Tests unitaires** : chaque module a ses tests.
* **Gestion des erreurs** : Result\<T, SqlError> partout.
* **Branches Git** : feature par module (`feature/core`, `feature/parser`).

---

## Roadmap

1. Core : types, tables, database, tests unitaires.
2. Lexer / Parser : génération d’AST.
3. Executor : CREATE / INSERT / SELECT.
4. Interface REPL.
5. Storage (JSON / Bincode).
6. Extensions : WHERE, UPDATE, DELETE, index.
