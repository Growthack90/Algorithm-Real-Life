# PAYPAL (Flixbus)

## Problema
Creare logica di flusso che emula una transazione per l'acquisto di un ticket di viaggio Flixbus, utilizzando un servizio sicuro per le transazioni. \
Inoltre verrà aggiunta la funzionalità che in caso di un duplice acquisto dello stesso ticket (avvenuto a causa di una dimenticanza del cliente), allora il sistema congelerà il nuovo ticket acquistato e rimborserà l'utente.

## Soluzione
Nel codice di simulazione non ci saranno vere transazioni di pagamento, né un vero database persistente. I dati verranno mantenuti in memoria e persi alla chiusura del programma.

**Entità Coinvolte:**

* **Utente:** Il cliente che acquista il biglietto.
* **Sistema Flixbus (Frontend/Backend):** La piattaforma Flixbus con cui l'utente interagisce.
* **Database Viaggi:** Contiene informazioni su tratte, orari, prezzi, posti disponibili.
* **Database Biglietti Acquistati:** Memorizza i biglietti venduti e i dettagli dei passeggeri.
* **Servizio di Pagamento Sicuro (SPS):** Un gateway di pagamento esterno (es. Stripe, PayPal, Nexi) che gestisce i dati sensibili delle carte e le transazioni.

**Logica di Flusso:**

1.  **Inizio: Ricerca del Viaggio**
    * `Utente` → `Sistema Flixbus`: Inserisce criteri di ricerca (es. origine, destinazione, data partenza, numero passeggeri).
    * `Sistema Flixbus` → `Database Viaggi`: Interroga per viaggi corrispondenti.
    * `Database Viaggi` → `Sistema Flixbus`: Restituisce lista di viaggi disponibili (con dettagli: ID viaggio, orari, durata, prezzo, posti rimanenti).
    * `Sistema Flixbus` → `Utente`: Mostra i risultati della ricerca.

2.  **Selezione del Viaggio e Dettagli Passeggero**
    * `Utente` → `Sistema Flixbus`: Seleziona il viaggio desiderato (`ViaggioScelto`).
    * `Sistema Flixbus`: Verifica nuovamente la disponibilità di posti per `ViaggioScelto`.
        * **Se non disponibile:** Messaggio all'utente. Torna al punto 1.
        * **Se disponibile:** Prosegui. (Opzionale: blocco temporaneo del posto per X minuti).
    * `Utente` → `Sistema Flixbus`: Inserisce i dati del passeggero/i (nome, cognome, email, numero di telefono). L'email o il numero di telefono possono fungere da identificativo utente per il controllo duplicati.
    * `Sistema Flixbus`: Calcola il prezzo totale.

3.  **Riepilogo e Conferma per Pagamento**
    * `Sistema Flixbus` → `Utente`: Mostra il riepilogo dell'ordine (dettagli `ViaggioScelto`, dati passeggero, prezzo totale).
    * `Utente` → `Sistema Flixbus`: Conferma l'intenzione di procedere al pagamento.

4.  **Interazione con il Servizio di Pagamento Sicuro (SPS)**
    * `Sistema Flixbus` → `SPS`: Inizializza una transazione, inviando l'importo e un riferimento univoco dell'ordine. NON invia dati sensibili della carta.
    * `Sistema Flixbus` → `Utente`: Reindirizza l'utente (o carica un iframe/componente) all'interfaccia del `SPS`.
    * `Utente` ↔ `SPS`: L'utente inserisce i dati di pagamento (numero carta, scadenza, CVV, etc.) direttamente nell'ambiente sicuro del `SPS`.
    * `SPS`: Processa la transazione (autenticazione 3D Secure, addebito).
    * `SPS` → `Sistema Flixbus`: Comunica l'esito della transazione (SUCCESSO o FALLIMENTO) e un codice di transazione univoco (`IDTransazioneSPS`). Non restituisce mai i dati completi della carta al Sistema Flixbus.

5.  **Post-Pagamento e Controllo Duplicati**
    * `Sistema Flixbus`: Riceve l'esito dal `SPS`.
    * **SE `EsitoPagamento` == FALLIMENTO:**
        * `Sistema Flixbus` → `Utente`: Notifica "Pagamento fallito. Riprova o usa un altro metodo."
        * (Opzionale: rilascia il posto temporaneamente bloccato).
        * Fine del flusso per questo tentativo.
    * **SE `EsitoPagamento` == SUCCESSO:**
        * `Sistema Flixbus`: **Avvia Controllo Acquisto Duplicato.**
            * Criteri per duplicato: Stesso `ID Viaggio`, stessa `Data Partenza`, stesso `Nome Passeggero` principale (o email/telefono dell'acquirente se associato al passeggero) e un biglietto `CONFERMATO` già esistente.
            * `Sistema Flixbus` → `Database Biglietti Acquistati`: Cerca un biglietto esistente (`BigliettoPrecedente`) che corrisponda ai criteri di duplicazione per l'utente/passeggero corrente.
            * **SE `BigliettoPrecedente` TROVATO (Rilevato Acquisto Duplicato):**
                1.  `Sistema Flixbus`: Contrassegna il *nuovo tentativo* di acquisto come "DUPLICATO_SOSPESO". Non genera un nuovo biglietto valido.
                2.  `Sistema Flixbus` → `SPS`: Invia una richiesta di **rimborso** per `IDTransazioneSPS` relativo all'acquisto duplicato appena effettuato.
                3.  `SPS` → `Sistema Flixbus`: Comunica l'esito del rimborso.
                4.  `Sistema Flixbus` → `Utente`: Notifica:
                    * "Attenzione: Abbiamo rilevato che hai già acquistato un biglietto identico per [Nome Passeggero] su questo viaggio ([Dettagli Viaggio]). Il tuo biglietto precedente ([ID Biglietto Precedente]) rimane valido."
                    * "L'importo per questo secondo acquisto ([Prezzo]) è stato rimborsato sulla tua carta (ID Transazione Rimborso: [IDTransazioneRimborsoSPS])."
                    * (Se il rimborso automatico fallisce, il messaggio informerà che il caso verrà gestito dal servizio clienti e il biglietto rimane "congelato".)
                5.  `Sistema Flixbus`: Logga l'evento (acquisto duplicato, rimborso effettuato/fallito).
                6.  Fine del flusso per questo tentativo.
            * **SE `BigliettoPrecedente` NON TROVATO (Nessun Duplicato):**
                1.  Procedi con l'emissione normale del biglietto.

6.  **Emissione del Biglietto (se non duplicato e pagamento OK)**
    * `Sistema Flixbus`: Genera un ID univoco per il nuovo biglietto (`IDNuovoBiglietto`).
    * `Sistema Flixbus` → `Database Biglietti Acquistati`: Salva i dettagli del `NuovoBiglietto` (associandolo a `IDViaggio`, dati passeggero, `IDTransazioneSPS`, stato: `CONFERMATO`).
    * `Sistema Flixbus` → `Database Viaggi`: Decrementa il numero di posti disponibili per `ViaggioScelto`.
    * `Sistema Flixbus` → `Utente`: Mostra messaggio di conferma acquisto: "Pagamento completato! Il tuo biglietto [IDNuovoBiglietto] è confermato."
    * `Sistema Flixbus`: Invia email di conferma all'utente con il biglietto elettronico (PDF o QR code) e tutti i dettagli.

7.  **Fine della Transazione**

**Considerazioni Aggiuntive:**

* **Identificazione Utente:** Per il controllo duplicati, è fondamentale un modo affidabile per identificare l'utente che ha effettuato l'acquisto (es. login, email) e/o il passeggero principale.
* **Definizione di "Stesso Ticket":** È cruciale definire chiaramente cosa costituisce un duplicato. Tipicamente: stessa tratta, data, ora e stesso passeggero principale. Se un utente acquista due biglietti per lo stesso viaggio ma per due passeggeri diversi, non è un duplicato.
* **Finestra Temporale per Duplicati:** Il controllo duplicati è più efficace se considera acquisti recenti o attivi. Un acquisto identico fatto un anno prima per un viaggio passato non è rilevante.
* **Errori nel Rimborso:** Se il rimborso automatico al `SPS` fallisce, il sistema deve avere un meccanismo di alerting per intervento manuale del servizio clienti. Il biglietto "congelato" non dovrebbe diventare attivo.
* **Concorrenza:** Se più richieste arrivano contemporaneamente per gli ultimi posti, il sistema deve gestire la concorrenza per evitare overbooking (solitamente tramite transazioni atomiche sul database o meccanismi di locking).
* **Comunicazione Utente:** I messaggi all'utente devono essere chiari, specialmente in caso di rilevamento di duplicati e rimborsi.

Questo flusso logico copre i requisiti principali, ponendo l'accento sulla sicurezza delle transazioni e sulla gestione proattiva degli acquisti accidentali duplicati.