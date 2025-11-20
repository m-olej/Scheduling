# Problem 1
> 1 | r_j, S_ij | Sum C_j


## Specification
Problem szeregowania na jednej maszynie

`n` zadań do wykonania = {J_1, ..., J_n}

Każde zadanie jest opisane przez `p_j` (czas trwania) oraz `r_j` (moment gotowości)

Dla każdej pary zadań `(J_i, J_j)` zdefiniowany jest czas przezbrojenia `S_ij`, czyli ile czasu jest wymagane przed rozpoczęciem zadania J_j po zakończeniu zadania J_i. `S_ii = 0`

Zadanie nie może rozpocząć się przed swoim momentem gotowości: `r_j <= C_j - p_j`

Zadania wykonywane są bez przerwy

`Kryterium`:
Suma momentów czasów zakończenia zadań `Sum C_j`

## Example program usage

1. Program named 155927

```
155927 in_123456_100.txt out_155927_123456_100.txt
```

## Example input file
> in_155927_rozmiar.txt - n=10 -> in_155927_10.txt
n
p_1 r_1
p_2 r_2
...
S_11 S_12 ... S_1n
S_21 ...
...
S_n1 ...

*tips*:
* uniemożliwić posortowanie po `p_i`
* `p_i` - niezbyt duże (1-30)
* `S_ij` - niezbyt duże (1-30)
* `r_i` dla małych `p_i` powinny być dalekie 
* `r_i` (0, suma wszystkich czasów wykonywania) 

## Example output file
> out_155927_album2_rozmiar.txt - n=10 -> out_155927_155347_10.txt
Sum C_j
J_1 J_2 ... J_n

## Limity

_czas_  
* `n/10` sekund, gdzie `n` to liczba zadań
* minimalny czas = `1ms`

_rozmiary instancji_
* n = [50, 500] co 50


---

*Strategia*:

Sortowanie po kluczach: `r_j`, `p_j` oraz `S_ij`. Powinno się zacząc od jak najkrótszych zadań takich, które szybko pozwolą na wykonywanie następnych krótkich zadań.

Priorytety:
- `r_j` najważniejsze, aby zacząć jak najszybciej. Chyba, że lepszy wynik da poczekanie na krótszy task
- nie warto w metaheurystyke

*compiling for OS*
* Windows
```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
* Linux
```
cargo build --release
```