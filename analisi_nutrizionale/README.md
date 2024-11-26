# ANALIZZATORE NUTRIZIONALE

## Descrizione 

Questo strumento da riga di comando ti permette di analizzare rapidamente il profilo nutrizionale di un alimento. Inserisci i valori nutrizionali per 100g di prodotto (calorie, grassi, carboidrati, ecc.) e l'app ti fornirà un'analisi semplice e immediata. 

## Problema

Valutare tramite inserimento dei valori nutrizionali richiesti degli elementi (calorie, grassi, zuccheri, fibre, proteine, sale, ecc.) ed aiutare a capire se un alimento è equilibrato o meno. Ideale per chi vuole monitorare la propria alimentazione in modo semplice e veloce.
Il risultato sarà l'analisi completa basata sul contronto del valore dell'elemento nutrizionale con il limite, restituendo il risultato.

**Note:**

* Questa app fornisce un'analisi di base e non sostituisce il parere di un esperto in nutrizione.
* I valori di riferimento utilizzati per l'analisi sono indicativi e potrebbero variare a seconda delle esigenze individuali.

## Soluzione
Steps:
0) impostare valori limite elementi nutritivi, e apporto nutrizionale massimo giornaliero.
1) Inserire valori nutritivi di riferimento (su 100g di prodotto)
2) Inserire valore assunto (in g)
3) Risultato
4) Il risultato al punto 3) è cumulabile per altri apporti che avvengono durante la giornata.

**Esempio Output ideale:**
```
Inserisci i valori nutrizionali per 100g:
Calorie (kcal): 474
Grassi (g): 18.5
Grassi saturi (g): 4
Carboidrati (g): 67.3
Zuccheri (g): 15.2
Fibre (g): 4
Proteine (g): 7.5
Sale (g): 0.925

Analisi:
Questo alimento ha un profilo nutrizionale bilanciato.
```

---
## Valori di riferimento (esempio)
Ecco i range ottimali per 100g di questo prodotto, basandoci sulle informazioni nutrizionali forniti e su linee guida generali per una dieta bilanciata:

**Energia:** Idealmente, un alimento dovrebbe apportare meno di 250 kcal per 100g per essere considerato "ipocalorico". Questo prodotto ha 474 kcal per 100g, quindi è piuttosto calorico.

**Grassi:**  Un buon range per i grassi è tra 10 e 20g per 100g. Questo prodotto ha 18.5g di grassi, quindi rientra nella parte alta del range. Sarebbe ideale se una porzione maggiore di questi grassi fosse insatura.

**Acidi grassi saturi:**  Dovrebbero essere il più bassi possibile. 4g per 100g è un valore accettabile, ma sarebbe meglio se fosse inferiore a 3g.

**Carboidrati:** Un range ideale per i carboidrati è tra 50 e 65g per 100g. Questo prodotto ha 67.3g, quindi leggermente sopra il range ideale.

**Zuccheri:**  È consigliabile limitare gli zuccheri a meno di 25g per 100g. 15.2g per 100g è un valore accettabile.

**Fibre:**  Un buon apporto di fibre è importante per la salute. 4g per 100g è un valore discreto, ma sarebbe meglio se fosse superiore a 6g.

**Proteine:**  Un buon range per le proteine è tra 7 e 15g per 100g. 7.5g per 100g è un valore accettabile.

**Sale:**  È consigliabile limitare il sale a meno di 0.5g per 100g. 0.925g è un valore elevato.

**In sintesi:** Questo prodotto sembra essere abbastanza calorico e ricco di grassi, con un contenuto di sale elevato.  Potrebbe essere consumato occasionalmente come parte di una dieta bilanciata, ma non è l'ideale per un consumo frequente.

**Ricorda:** Questi sono solo range generali. Le tue esigenze nutrizionali specifiche possono variare a seconda di diversi fattori come età, sesso, livello di attività fisica e stato di salute. È sempre meglio consultare un professionista della salute o un dietologo per una valutazione personalizzata.

----

## Come contribuire

I contributi sono benvenuti! Se vuoi contribuire a questo progetto, puoi:

* Segnalare bug o richiedere nuove funzionalità aprendo una issue.
* Inviare pull request con correzioni di bug o nuove funzionalità. Assicurati di seguire le linee guida per i contributori (CONTRIBUTING.md).
* Aiutare a migliorare la documentazione.