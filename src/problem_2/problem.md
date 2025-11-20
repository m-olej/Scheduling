# Problem 2
> Q5 | r_j | Sum_Yj

* 5 maszyn równoległych jednorodnych, opisanych współczynnikiem prędkości $𝑏_𝑘$
wskazującym ile razy maszyna $𝑀_𝑘$ jest wolniejsza od najszybszej maszyny w systemie
(co najmniej jeden ze współczynników $𝑏_𝑘$ musi wynosić 1)
* 𝑛 zadań do wykonania $𝐽_1, … , 𝐽_𝑛$
* każde zadanie $𝐽_𝑗$ opisane jest czasem trwania $𝑝_𝑗$ i momentem gotowości $𝑟_𝑗$ i oczekiwanym
terminem zakończenia wykonywania $𝑑_𝑗$
* należy przydzielić zadania do maszyn i ustalić kolejność wykonania na maszynach
minimalizując całkowitą pracę spóźnioną ∑ 𝑌𝑗, gdzie 𝑌𝑗 = min{max{𝐶𝑗− 𝑑𝑗, 0} , 𝑝𝑗} oznacza
pracę spóźnioną
* zadanie nie może rozpocząć się przed swoim momentem gotowości 𝑟𝑗 ≤ 𝐶𝑗 − 𝑝𝑗
* zadania wykonywane są bez przerwań na przydzielonej maszynie

### Example input
```
n
b_1, b_2, b_3, b_4, b_5
p_1, r_1, d_1
...
p_n, r_n, d_n
```
---
```
4
1.0 1.2 1.4 1.9 1.5
4 2 3
2 0 5
3 5 10
3 4 10
```

### Example output
```
Sum_Yj
J_1,1 J_1,2 ...
J_2,1 J_2,2 ...
...
J_5,1 J_5,2 ...
```

## Ustalenia
* n = $[50, 500]$ z krokiem o 50
* b_k = $[1, 2)$ z krokiem o 0.1 
* używać kropek przy floatach (np..: 1.5)
* Limit czasu: $n/10$ sekund
* mogą być > 1; b_k = 1
* program ma liczyć na floatach, ale do wyniki całkowite (post-factum)
