# Algorithm Real Life Rust

**Benvenuto nel mio "Museo del Codice"!**

Questo repository è una raccolta di piccoli progetti che rappresentano cose che ho incontrato e che hanno suscitato la mia curiosità. \
Qui troverai esperimenti di codice che esplorano:

* **Sistemi del mondo reale:** Come funziona la logica di strumenti di uso quotidiane.
* **Concetti di programmazione:** Implementazioni di algoritmi, strutture dati, e pattern di design.
* **Sfide divertenti:** Giochi, enigmi, e altre creazioni nate dalla voglia di sperimentare.

**Perché questo repository?**

* **Imparare facendo:** Il modo migliore per capire qualcosa è provarla. Questi progetti sono il risultato del mio desiderio di imparare attraverso la pratica.
* **Condividere la conoscenza:** Spero che questi esempi possano essere utili ad altri che stanno esplorando gli stessi argomenti.
* **Divertirsi:** Programmare dovrebbe essere anche un piacere! 

**Come esplorare:**

* Dai un'occhiata alle cartelle individuali per ogni progetto.
* Leggi i commenti nel codice per capire il mio ragionamento.
* Sentiti libero di sperimentare, modificare e migliorare!

**Nota:** Questi progetti sono principalmente a scopo didattico. Potrebbero non essere le soluzioni più efficienti o eleganti, ma rappresentano il mio percorso di apprendimento.

**Buon divertimento esplorando!** 

**Note Aggiuntive:**

Ogni progetto ha una breve descrizione con le istruzioni su come eseguirlo

## Uso di una Workspace
I vari progettini sono raccolti all'interno di una [**Workspace**](https://doc.rust-lang.org/cargo/reference/workspaces.html), lavorando con una repository strutturata per gestire più progetti Rust in modo coordinato.

Una workspace di Cargo permette di gestire più progetti Rust (detti anche "crate") correlati all'interno di una singola repository. Questo semplifica la compilazione, il testing e la gestione delle dipendenze tra i vari progetti.

### Vantaggi dell'utilizzo di una workspace

- **Compilazione centralizzata**: Tutti i progetti nella workspace vengono compilati insieme, garantendo che siano compatibili tra loro.
- **Dipendenze condivise**: Le dipendenze comuni possono essere specificate una sola volta a livello di workspace, evitando duplicazioni e semplificando la gestione.
- **Esecuzione di test**: I test per tutti i progetti possono essere eseguiti con un singolo comando.


### Note aggiuntive
Dove uno o più membri del workspace (cioè i progetti inclusi) utilizzano l'edizione Rust 2021 o successiva, Cargo usa un **resolver** per gestire le dipendenze del tuo progetto.\
Il resolver versione "2" è stato introdotto con Rust 2021 e offre una migliore gestione delle funzionalità (features) delle dipendenze, risolvendo potenziali problemi che potevano sorgere con il resolver versione "1".\
Quando un membro del workspace usa l'**Edizione Rust 2021** o successiva, si aspetta implicitamente che venga utilizzato il resolver "2".

**Cosa dovresti fare?**

Cargo ti suggerisce due opzioni:

1. **Mantenere il resolver attuale (`resolver = "1"`):**
   * Aggiungi la seguente riga al `Cargo.toml` di **ogni** membro del workspace:
     ```toml
     [workspace]
     resolver = "1"
     ```
   * Questo manterrà il comportamento precedente, ma potresti incontrare problemi con le funzionalità delle dipendenze se il tuo progetto è complesso.

2. **Usare il resolver per l'edizione 2021 (`resolver = "2"`):**
   * Aggiungi la seguente riga al `Cargo.toml` di **ogni** membro del workspace:
     ```toml
     [workspace]
     resolver = "2"
     ```
   * Questa è l'opzione consigliata, in quanto risolve potenziali problemi e si allinea alle aspettative dell'edizione 2021.

**In sintesi:**

* Se il tuo progetto è semplice e non hai problemi con le dipendenze, puoi scegliere di mantenere il resolver "1".
* Se vuoi sfruttare appieno le funzionalità di Rust 2021 e successive, o se incontri problemi con le dipendenze, dovresti passare al resolver "2". 

**Per ulteriori informazioni:**

* Consulta la documentazione ufficiale di Cargo sulle [versioni del resolver](https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions):


## Elenco Progetti
- App Dating
- Distributore Automatico
- PayPal (acquisto biglietto Flixbus)
- Antivirus
- Analisi Nutrizionale