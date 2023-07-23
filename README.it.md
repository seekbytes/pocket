# POCKET

[POCKET](https://github.com/seekbytes/pocket) è un programma che consente di applicare delle trasformazioni per offuscare una espressione MBA (Mixed Boolean Arithmetic). Accetta delle espressioni in input e le offusca tramite alcune regole che vengono applicate per sostituzione. 

Esempi di regole che vengono applicate:

```
X ^ Y == (X | Y) - (X & Y)
X + Y == (X & Y) + (X | Y)
X - Y == (X ^ -Y) + 2*(X & -Y)
X & Y == (X + Y) - (X | Y)
X | Y == X + Y + 1 + (~X | ~Y)
``` 

## Rendere difficile l'analisi statica

La maggior parte dei decompilatori (ad esempio IDA Pro, Ghidra, Binary Ninja) sono in grado di riottenere da un qualsiasi programma binario gran parte delle istruzioni originali. Seppur le istruzioni in codice macchina non contengano vaste informazioni (come commenti, nomi di variabili, strutture di alto livello), è ancora possibile recuperare gran parte della logica originale.

Rendere difficile l'analisi effettuata dai decompilatori è un compito che si basa sul concetto di offuscamento. Dato un programma P in input, l'operazione di offuscamento cerca di applicare alcune tecniche per complicare la logica del programma, ostacolando il lavoro effettuato dai decompilatori. Una delle tecniche più semplici da implementare è la tecnica dell'offuscamento delle operazioni MBA.

Le espressioni MBA sono espressioni che comprendono operazioni logiche e aritmetiche. Esempi di espressioni MBA sono dati dal risultato dei decompilatori che cercano di trasformare una sequenza di operazioni in linguaggio macchina in una espressione logica, aritmetica. Generalmente le espressioni MBA vengono utilizzate per codificare un numero o una condizione logica da parte dell'ottimizzatore del compilatore.

```
rax = 0x8007afb + (0x780 >> 2) | 0x87
```

Le trasformazioni vengono applicate riscrivendo l'albero della sintassi costruito a partire dall'espressione. La riscrittura è semplicemente una visita dell'albero dal nodo radice che applica ricorsivamente le regole di trasformazioni per offuscare i singoli nodi. Ad esempio, l'espressione `A + (B & C)` produce un albero sintattico composto da:

```
.
|- +
|--- A
|--- &
|------ C
|------ D
```

Applichiamo la trasformazione `LEFT + RIGHT = (LEFT & RIGHT) + (LEFT | RIGHT)`. Dobbiamo ovviamente stare attenti a cosa abbiamo a sinistra e a destra. Per l'esempio `LEFT` è `A`, mentre `RIGHT` è `C & D`. La trasformazione riscrive l'espressione, facendola diventare: `(A & (C & D)) + (A | (C & D))`. Fine. Applicando le diverse trasformazioni e rendendole ancora più "pesanti" algebricamente parlando (il valore `1` può essere riscritto in infiniti modi, incluso `(((-1283928202102 & 1283928202103) << 2) >> 2) + (((-1283928202102 | 1283928202103) << 2) >> 2)`). Non c'è limite alla fantasia!

Il trucco poi sta nell'utilizzare il risultato del primo offuscamento (livello #0) come input per il successivo livello di offuscamento e avanti così. È possibile continuare ad eseguire delle "passate" sull'espressione originale per arrivare ad una espressione sempre più complessa (e con molti più nodi!). Tutto però ha un prezzo: aumentando il livello di offuscamento, aumenta sia il tempo (ci vuole più tempo per l'attraversamento dell'albero e per la riscrittura) sia lo spazio (l'abstract syntax tree occupa più byte, banalmente). Per ragioni a me ignote, su grandi espressioni, il progetto fa fatica ad applicare le trasformazioni.

Per verificare se una espressione offuscata sia semanticamente uguale all'espressione originale, ho scritto un semplice interprete che prevede di valutare l'espressione come una sorta di calcolatrice. Se il risultato ottenuto dalla valutazione dell'espressione offuscata è uguale al risultato dell'espressione originale, allora la trasformazione è valida e l'espressione è stata offuscata correttamente. Il valore del SET viene determinato dal valore nella tabella ASCII del nome del SET (ad esempio 'A' è 65). 

## Compilare ed eseguire il progetto

Il progetto è scritto in Rust utilizzando il parser [Pest](https://pest.rs). Pest consente di scrivere una grammatica utilizzando la sintassi per le [grammatiche delle espressioni dei parser](https://en.wikipedia.org/wiki/Parsing_expression_grammar) (PEG). La grammatica del linguaggio è disponibile nel file `src/grammar.pest` e descrive il tipo di espressioni accettate all'interno del programma. Qui riassumiamo un paio di regole:

- una espressione è composta da una o una serie di operazioni logiche e arimetiche descritte così:
	- operazioni binarie: SET OP SET, dove SET è una lettera dell'alfabeto ASCII mentre OP è tra – (sottrazione), + (addizione), | (OR), & (AND), ^ (XOR)
	- operazione unaria: OP SET, dove SET è una lettera dell'alfabeto ASCII mentre OP è tra ~ (NOT) e - (operazione di negazione)

Per compilare il progetto è sufficiente utilizzare il comando `cargo build`, troverete il binario all'interno di `./target/debug/mfa`. Per eseguirlo invece `cargo run`.

## Perché il progetto si chiama POCKET?

Il nome POCKET ("tasca" in italiano) deriva da una pillola di vita quotidiana. Quante volte ci è capitato di inserire una matassa di un filo (auricolari, caricabatterie) all'interno di una tasca e dopo averlo estratto ci siamo ritrovati con un filo da srotolare? Troppe volte.

Ecco, immaginiamo di avere una espressione (il filo) e di volerla rendere più complessa da visualizzare. Possiamo utilizzare POCKET (tasca) per avere una espressione più complessa in modo magico. Semplicemente la inseriamo dentro e dopo una manciata di microsecondi abbiamo la nostra espressione più complessa.

Vabbè, non avevo altri titoli e questa mi sembrava una buona idea.