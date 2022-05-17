# mariage_stable_rust:

Ce repository, contient le code lié à mon stage de Tutorship DS4H, effectué durant ma première année de Master à UCA (Université Côte d'Azur).
Il se divise en deux partie, un solveur du problème des mariages stables (en Rust) et un harnais de test de performances (en Python).

## Prérequis:
### Le solveur:
  - [Rust (Latest)](https://www.rust-lang.org/tools/install "Rust's Installation")
### La génération de graphique:
  - [Python (Latest)](https://www.python.org/downloads/ "Python's Download")
  - [Pip](https://pip.pypa.io/en/stable/installation/ "Pip's Installation")
  - [Numpy](https://numpy.org/install/ "Numpy's Installation")
  - [Matplotlib](https://matplotlib.org/stable/users/getting_started/index.html#installation-quick-start/ "Matplotlib's Installation")

## Comment lancer:
```shell
USAGE:
    cargo run -- [OPTIONS]

OPTIONS:
    -h, --help                                         Print help information
        --instance-size-end <INSTANCE_SIZE_END>        [default: 5]
        --instance-size-start <INSTANCE_SIZE_START>    [default: 5]
    -n, --number-repetition <NUMBER_REPETITION>        [default: 1]
    -p, --pas <PAS>                                    [default: 1]
    -s, --seed <SEED>                                  [default: 0]
    -t, --thread-number <THREAD_NUMBER>                [default: 4]
    -V, --version                                      Print version information
    -w, --worst-case                                   
```
Voilà un exemple d'utilisation pour tester notre solveur sur des instances allant de la taille 10 à 5000 avec un pas de 10, répétant 500 fois l'expérience en utilisant 4 threads pour l'algorithme parallèle:
```shell
cargo run -- --instance-size-start 10 --instance-size-end 5000 -p 10 -t 4 -n 500
```
