# BST vs TreapRec vs Treap

Dieses Programm ermöglicht es einen `Binary Search Tree (BST)` mit zwei
Implementierungen eines `Treap`s zu vergleichen. Eine Implementierung geht
rekursiv durch den Baum (`TreapRec`), die andere iterativ (`Treap`).

## Installation

Die Implementierung ist in der Programmiersprache Rust. Dafür muss der Compiler
installiert sein. Die Anleitung finden Sie
[hier](https://www.rust-lang.org/tools/install).

Wenn Rust installiert ist, kann im Projektordner (mit `Cargo.toml` Datei) das
Programm ausgeführt werden. Führen Sie dafür einfach `cargo run --release` in
der Konsole aus.

## Bedienung

Nach dem Start des Programms stehen sechs Befehle zur Verfügung:

### insert

Ermöglicht es einen neuen Eintrag in jeder Datenstruktur einzufügen. Dafür sind
zwei Eingaben nötig, ein Schlüssel und einen Wert. Für die Treaps wird ein
zufälliges Gewicht ausgewählt.

**Beispiel:**

```
> Enter a command (insert | find | print | time | load | exit):
insert
Enter english word:
dog
Enter german word:
Hund
```

### find

Ermöglicht die Suche mit Hilfe eines Schlüssels. Für jede der drei
Datenstrukturen wird dann entweder `true <Wert>` oder `false` zurückgegeben, je
nachdem ob es gefunden wurde oder nicht. Die Reihenfolge ist `Treap`,
`TreapRec`, `BST`.

```
> Enter a command (insert | find | print | time | load | exit):
find
> Enter english word to find:
dog
true Hund
true Hund
true Hund
```

### print

Gibt die Datenstruktur in der Konsole aus.

### time

Misst die Zeit in Nanosekunden **für den nächsten find oder insert Befehl**.

**Beispiel:**

```
> Enter a command (insert | find | print | time | load | exit):
time
>> The next operation will be timed
> Enter a command (insert | find | print | time | load | exit):
find
> Enter english word to find:
dog
Previous operation for Treap completed in 2146ns
true Hund
Previous operation for TreapRec completed in 406ns
true Hund
Previous operation for BST completed in 495ns
true Hund
```

### load

Lädt die angegebene Anzahl an Wörtern aus der gegebenen Wortliste (sortiert oder
unsortiert). Welche Wortliste gewählt werden soll kann im Quellcode in
`src/main.rs` angepasst werden, indem die folgende Zeile angepasst wird:

```rust
// ...

fn main() {
	// ...

	// kann zu WORDS_SORTED geändert werden.
	let mut words_iter = WORDS_UNSORTED.into_iter();

	// ...
}

// ...
```

Jede dieser Wortlisten besteht aus 58110 Wörtern. Mit jedem Aufruf von `load`
werden `n` neue Wörter geladen, bis die Wortliste ausgeschöpft ist.

**Beispiel:**

```
> Enter a command (insert | find | print | time | load | exit):
load
How many to load:
30000
>> Loaded 30000 words
> Enter a command (insert | find | print | time | load | exit):
load
How many to load:
30000
>> Loaded 28110 words
```

### exit

Beendet das Programm.

## Auswertung

Laufzeiten in Nanosekunden. Die Tests wurden wie im folgenden Beispiel
ausgeführt:

```sh
cat testdata/timed_find_[n].txt | cargo run --release
```

Aus den 4 Zeiten wurde dann der Durchschnitt gebildet.

### Sortierte Eingabe

| Anzahl der Wörter | BST      | Treap   | TreapRec |
| ----------------- | -------- | ------- | -------- |
| 10                | 128.5    | 157.75  | 108      |
| 100               | 855.5    | 176.5   | 127.75   |
| 1000              | 8969.5   | 342.25  | 353      |
| 10000             | 119493.5 | 847     | 928.25   |
| 58110             | 869587   | 2228.75 | 2204     |

Die Laufzeit des BST steigt deutlich schneller als die der Treaps, da die
Eingabe sortiert und somit der BST im Endeffekt wie eine Linked-List aussieht.
Die Treaps liegen sehr nah bei einander. Der Overhead durch die Rekursion
zeichnet sich hier kaum ab.

### Randomisierte Eingabe

| Anzahl der Wörter | BST      | Treap   | TreapRec |
| ----------------- | -------- | ------- | -------- |
| 10                | 144.25   | 238.75  | 162.25   |
| 100               | 205      | 307.5   | 263.25   |
| 1000              | 537.75   | 664.25  | 554.5    |
| 10000 xx          | 119493.5 | 847     | 928.25   |
| 58110             | 869587   | 2228.75 | 2204     |

### Vergleich Treap & TreapRec bei 50.000.000 Einträgen

| Treap   | TreapRec |
| ------- | -------- |
| 87323ns | 4501ns   |

Dieses Ergebnis zeigt, meine iterative Implementierung ist deutlich langsamer
als die rekursive. Ich erkläre mir dieses Ergebnis durch die zur Laufzeit
durchgeführten Ownership-Checks von `std::cell::RefCell` und dem Clonen von
`std::rc::Rc`'s, welche durch `unsafe` umgangen werden können, allerdings leicht
Undefined Behaviour verursachen könnte. Möglicherweise ist meine Vorgehensweise
auch nicht optimal.
