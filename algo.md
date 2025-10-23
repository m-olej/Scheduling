# Algorithm
> Hybrid VNS with GRASP

## Motive

The problem presents significant challanges when looking for a simple way to find the optimum solution 
as each parameter affects the rest in a negative way. 

Hence a greedy or simple heuristic algorithm will not produce satisfactory results.

A metaheuristic algorithm needs to be implemented to 
strategically search through the solution space and find the local minimum of the target function.

Given the significant variability in expected optimal parameter choice the optimal algorithm should make sure to 
explore a broad spectrum of solution paths before narrowing its focus.

The `VNS` (Variable Neighbour Search) should be a viable option as its mechanics enable it to 
explore the solution space broadly (exploration) as well as tightly (intensification).

`VNS` is very dependent on the initial starting point. 
This point should be picked using a simpler yet more effective than a greedy heuristic algorithm to evaluate starting points 
and pick the first local minimum it reaches. 
For this the `GRASP` (Greedy Randomized Adaptive Search Procedure) algorithm is picked for its ability to find
different high-value solutions per run, which will help `VNS` search through more of the solution space

---
The problem is most similar to a `TDTRP-TW` (Time Dependant Travelling Repair Problem with Time Windows)

---


## Architecture
> Multiphase algorithm

### Notation
`schedule` - sequence of tasks
`k` - sequence position
`n` - number of tasks
`target function` - Sum C_j
`f(schedule)` -> Value of target function 

1. Picking the starting point | `GRASP`
    * Start with an empty sequence
    * repeat until there are no unscheduled tasks:
        * For each unscheduled task and each possible insertion position in current sequence 
          calculate the target function increase 
        * Create the `RCL` (Restricted Candidate List) using the top `alpha %` of insertions
        * Pick a random candidate from the `RCL` and append to the schedule
2. Searching solution space | `VNS`
* Init:
    * Use the phase 1 solution as input
    * Create the Neighbourhood structure `N_k`
* Loop until stopping criteria: `time_limit`, `iteration_without_improvement_limit`
    * Set `k` = 1 
    * Until `k` <= `n`
        * **Shaking** -> Generate a new solution `s'` by applying `k` random moves to solutions from `N_k` adjacent to the latest best solution (`s_best'`)
        * **Intensification** -> Use the `VND` (Variable Neighbourhood Descent) procedure to the new solution which generates the new local optimum (`s''`)
        * **Neighbourhood change** -> If `f(s'')` < `f(s_best)` 
            * *then* set `s_best` = `s''` and reset `k` to 1
            * *else* increment `k` 
* return `s_best`

#### Neighbourhood

1. Swap
    * swap positions of two tasks
2. Relocate
    * Get task from position `k` and puts it a random `k'`
3. 2-Opt
    * Picks 2 `edges` (`k`, `k+1`), (`k', k'+1`) and changes them to (`k`, `k'`), (`k+1`, `k'+1`)
4. Block move
    * Moves a cohesive block of 2 or 3 tasks from one place in a sequence to a different one

```
FUNKCJA WygenerujRozwiazaniePoczatkowe(Zadania, CzasyPrzezbrojen):
    pi = // Pusta sekwencja początkowa
    nieuszeregowane = {wszystkie indeksy zadań}
    
    DOPÓKI nieuszeregowane jest niepuste:
        lista_kandydatow =
        DLA kazdego zadanie_idx w nieuszeregowane:
            DLA kazdej pozycji_wstawienia od 0 do len(pi):
                // Utwórz tymczasową sekwencję przez wstawienie zadania
                pi_temp = wstaw(pi, zadanie_idx, pozycja_wstawienia)
                
                // Oblicz koszt (wartość funkcji celu) dla tej tymczasowej sekwencji
                koszt = ObliczWartoscFunkcjiCelu(pi_temp, Zadania, CzasyPrzezbrojen)
                
                dodaj (zadanie_idx, pozycja_wstawienia, koszt) do lista_kandydatow
            KONIEC DLA
        KONIEC DLA
        
        // Utwórz Ograniczoną Listę Kandydatów (RCL)
        posortuj lista_kandydatow wg kosztu rosnąco
        RCL = pierwsze ALPHA * len(lista_kandydatow) elementów z posortowanej listy
        
        // Wybierz losowo kandydata z RCL
        (wybrane_zadanie, wybrana_pozycja, _) = losowy_element_z(RCL)
        
        // Zaktualizuj sekwencję i zbiór zadań nieuszeregowanych
        pi = wstaw(pi, wybrane_zadanie, wybrana_pozycja)
        usun wybrane_zadanie z nieuszeregowane
    KONIEC DOPÓKI

    ZWRÓĆ pi

FUNKCJA HybrydowyVNS_Harmonogramista(Zadania, CzasyPrzezbrojen, WarunekStopu):
    // Faza I: Generowanie rozwiązania początkowego
    pi_poczatkowe = WygenerujRozwiazaniePoczatkowe(Zadania, CzasyPrzezbrojen)
    pi_best = pi_poczatkowe
    wartosc_best = ObliczWartoscFunkcjiCelu(pi_best, Zadania, CzasyPrzezbrojen)

    // Faza II: Główna pętla VNS
    DOPÓKI WarunekStopu nie jest spełniony:
        k = 1
        DOPÓKI k <= k_max:
            // Krok 1: Wstrząs (Shaking)
            // Zaburz najlepsze dotychczasowe rozwiązanie
            pi_prim = Wstrzas(pi_best, k, Sasiedztwa[k])

            // Krok 2: Przeszukiwanie lokalne (VND)
            pi_bis = ZmiennePrzeszukiwanieLokalne(pi_prim, Zadania, CzasyPrzezbrojen)
            wartosc_bis = ObliczWartoscFunkcjiCelu(pi_bis, Zadania, CzasyPrzezbrojen)

            // Krok 3: Zmiana sąsiedztwa
            JEŚLI wartosc_bis < wartosc_best:
                pi_best = pi_bis
                wartosc_best = wartosc_bis
                k = 1 // Sukces, wróć do pierwszego sąsiedztwa
            INACZEJ:
                k = k + 1 // Porażka, spróbuj silniejszego wstrząsu
            KONIEC JEŚLI
        KONIEC DOPÓKI
    KONIEC DOPÓKI

    ZWRÓĆ (pi_best, wartosc_best)

4.2. Funkcja Celu: ObliczWartoscFunkcjiCelu

FUNKCJA ObliczWartoscFunkcjiCelu(pi, Zadania, CzasyPrzezbrojen):
    suma_Cj = 0
    czas_zakonczenia_poprzedniego = 0
    poprzednie_zadanie_idx = 0 // Indeks zadania "zerowego"

    DLA kazdego zadanie_idx w pi:
        zadanie = Zadania[zadanie_idx]
        p_j = zadanie.p
        r_j = zadanie.r
        S_ij = CzasyPrzezbrojen[poprzednie_zadanie_idx][zadanie_idx]

        // Oblicz czas rozpoczęcia, uwzględniając datę gotowości i czas przezbrojenia
        czas_rozpoczecia = max(r_j, czas_zakonczenia_poprzedniego + S_ij)
        
        // Oblicz czas zakończenia bieżącego zadania
        czas_zakonczenia_biezacego = czas_rozpoczecia + p_j
        
        suma_Cj = suma_Cj + czas_zakonczenia_biezacego
        
        // Zaktualizuj zmienne na potrzeby następnej iteracji
        czas_zakonczenia_poprzedniego = czas_zakonczenia_biezacego
        poprzednie_zadanie_idx = zadanie_idx
    KONIEC DLA

    ZWRÓĆ suma_Cj

FUNKCJA Wstrzas(pi, k, sasiedztwo):
    pi_prim = kopia(pi)
    // Zastosuj k losowych ruchów z danego sąsiedztwa
    DLA i od 1 do k:
        pi_prim = wykonaj_losowy_ruch(pi_prim, sasiedztwo)
    KONIEC DLA
    ZWRÓĆ pi_prim

FUNKCJA ZmiennePrzeszukiwanieLokalne(pi):
    pi_biezace = kopia(pi)
    DOPÓKI prawda:
        znaleziono_poprawe = fałsz
        DLA kazdego sasiedztwo w:
            // Znajdź PIERWSZY ruch poprawiający w danym sąsiedztwie
            (pi_nowe, znaleziono) = PrzegladajSasiedztwo(pi_biezace, sasiedztwo)
            
            JEŚLI znaleziono:
                pi_biezace = pi_nowe
                znaleziono_poprawe = prawda
                PRZERWIJ // Wróć do pierwszego sąsiedztwa (N_1)
            KONIEC JEŚLI
        KONIEC DLA
        
        JEŚLI nie znaleziono_poprawe:
            PRZERWIJ // Osiągnięto minimum lokalne względem wszystkich sąsiedztw
        KONIEC JEŚLI
    KONIEC DOPÓKI
    ZWRÓĆ pi_biezace

FUNKCJA PrzegladajSasiedztwo(pi, typ_sasiedztwa):
    // Ta funkcja systematycznie przeszukuje całe sąsiedztwo danego typu
    // i zwraca PIERWSZE znalezione rozwiązanie, które jest lepsze.
    // Przykładowa implementacja dla sąsiedztwa Swap:
    
    wartosc_biezaca = ObliczWartoscFunkcjiCelu(pi)
    
    DLA i od 0 do len(pi)-1:
        DLA j od i+1 do len(pi)-1:
            pi_sasiad = wykonaj_swap(pi, i, j)
            wartosc_sasiada = ObliczWartoscFunkcjiCelu(pi_sasiad)
            
            JEŚLI wartosc_sasiada < wartosc_biezaca:
                ZWRÓĆ (pi_sasiad, prawda) // Znaleziono poprawę
            KONIEC JEŚLI
        KONIEC DLA
    KONIEC DLA
    
    ZWRÓĆ (pi, fałsz) // Nie znaleziono poprawy
```






