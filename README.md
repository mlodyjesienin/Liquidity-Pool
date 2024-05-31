# Liquidity-Pool
Główna część projektu znajduje się w pliku `liquidity-pool/src/main.rs`.  
W pliku `liquidity-pool/src/decimal.rs` znajduje się moja implementacja klasy odpowiedzialnej za "fixed-point decimals". 

### Decimals
Na samym początku przy inicjalizacji Liquidity Pool, ustanawiana jest stała wartość dokładności (skala). Każda liczba Decimal to po prostu liczba przemnożona przez $10^skala$. Dla wygody zaimplementowane są tam cechy odpowiedzialne za przeciążenie operatorów dodawania, odejmowania, dzielenia, mnożenia itp.  

Uwaga! przez fakt, że skala jest statyczną wartością, w obecnej implementajci nie da się np. stworzyć dwóch instancji liquidity pool z rózną skalą dokładności. Z tego samego powodu pokazuje, że nie przechodzi wszystkich testów. Każdy test osobno przechodzi, natomiast uruchomienie ich na wielu wątkach jednocześnie powoduje wielokrotne zmienianie wartości statycznej.  

Warto zauważyć, że argumenty funkcji, to dalej wartości typu f64. Polecenie sugerowało co innego, ale moim zdaniem nie da się rozsądnie tego zastąpić. Zwracane wartości są już typu Decimal.

### Testy 
Testy znajdują się w pliku `main.rs` pod funkcją `main()`. Powinny być w osobnym pakiecie, ale rust bardzo uprzykrzał mi życie przez fakt, że nazwałem projekt z użyciem `-`. Poddałem się  i wrzuciłem testy do maina.
