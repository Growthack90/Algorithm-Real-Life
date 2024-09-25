# Algorithm Real Life Rust

## Uso di una Workspace
I vari progettini sono raccolti all'interno di una [**Workspace**](https://doc.rust-lang.org/cargo/reference/workspaces.html), lavorando con una repository strutturata per gestire più progetti Rust in modo coordinato.

Una workspace di Cargo permette di gestire più progetti Rust (detti anche "crate") correlati all'interno di una singola repository. Questo semplifica la compilazione, il testing e la gestione delle dipendenze tra i vari progetti.

### Vantaggi dell'utilizzo di una workspace

- **Compilazione centralizzata**: Tutti i progetti nella workspace vengono compilati insieme, garantendo che siano compatibili tra loro.
- **Dipendenze condivise**: Le dipendenze comuni possono essere specificate una sola volta a livello di workspace, evitando duplicazioni e semplificando la gestione.
- **Esecuzione di test**: I test per tutti i progetti possono essere eseguiti con un singolo comando.

## Elenco Progetti