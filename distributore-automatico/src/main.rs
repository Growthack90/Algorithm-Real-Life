use std::io::{self, Write}; // Per input/output

// Struttura per rappresentare un prodotto nel distributore
#[derive(Debug, Clone)] // Clone è utile se vogliamo manipolare copie, Debug per stampe facili
struct Prodotto {
    id: u32,
    nome: String,
    prezzo: u32, // Prezzo in centesimi (es. 150 per 1.50 EUR)
    quantita: u32,
}

// Struttura per rappresentare il distributore automatico
struct Distributore {
    prodotti: Vec<Prodotto>,
    credito_inserito: u32, // Credito corrente inserito dall'utente, in centesimi
}

impl Distributore {
    // Costruttore per un nuovo distributore con prodotti predefiniti
    fn new() -> Self {
        Distributore {
            prodotti: vec![
                Prodotto { id: 1, nome: "Coca Cola".to_string(), prezzo: 150, quantita: 5 },
                Prodotto { id: 2, nome: "Patatine".to_string(), prezzo: 120, quantita: 3 },
                Prodotto { id: 3, nome: "Acqua Minerale".to_string(), prezzo: 80, quantita: 10 },
                Prodotto { id: 4, nome: "Snack Cioccolato".to_string(), prezzo: 100, quantita: 0 }, // Esaurito
                Prodotto { id: 5, nome: "Caffè Espresso".to_string(), prezzo: 60, quantita: 7 },
            ],
            credito_inserito: 0,
        }
    }

    // Mostra i prodotti disponibili
    fn mostra_menu(&self) {
        println!("\n--- DISTRIBUTORE AUTOMATICO ---");
        println!("Prodotti disponibili:");
        for prodotto in &self.prodotti {
            print!("ID: {} - {} (Prezzo: {:.2} EUR) - ", prodotto.id, prodotto.nome, (prodotto.prezzo as f32) / 100.0);
            if prodotto.quantita > 0 {
                println!("Disponibili: {}", prodotto.quantita);
            } else {
                println!("ESAURITO");
            }
        }
        if self.credito_inserito > 0 {
            println!("Credito inserito: {:.2} EUR", (self.credito_inserito as f32) / 100.0);
        }
        println!("-------------------------------");
    }

    // Permette all'utente di inserire monete
    fn inserisci_moneta(&mut self) {
        let monete_valide = [5, 10, 20, 50, 100, 200]; // centesimi (0.05, 0.10, 0.20, 0.50, 1.00, 2.00 EUR)
        println!("Inserisci il valore della moneta in centesimi (es. 50 per 0.50 EUR).");
        println!("Monete accettate (centesimi): {:?}", monete_valide);
        print!("Valore moneta: ");
        io::stdout().flush().unwrap(); // Assicura che il prompt venga stampato prima dell'input

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Errore lettura input.");

        match input.trim().parse::<u32>() {
            Ok(valore) => {
                if monete_valide.contains(&valore) {
                    self.credito_inserito += valore;
                    println!("Moneta da {:.2} EUR accettata. Credito totale: {:.2} EUR",
                             (valore as f32) / 100.0,
                             (self.credito_inserito as f32) / 100.0);
                } else {
                    println!("Moneta non valida. Valori accettati (in centesimi): {:?}.", monete_valide);
                }
            }
            Err(_) => {
                println!("Input non valido. Inserisci un numero intero.");
            }
        }
    }

    // Processa l'acquisto di un prodotto
    fn acquista_prodotto(&mut self, id_prodotto_scelto: u32) {
        // Trova l'indice del prodotto nell'array prodotti
        let indice_prodotto_opt = self.prodotti.iter().position(|p| p.id == id_prodotto_scelto);

        if let Some(indice) = indice_prodotto_opt {
            // Usiamo un blocco per limitare il prestito mutabile
            let mut puo_acquistare = false;
            let mut nome_prodotto_acquistato = String::new();
            let mut resto_da_dare = 0;

            // Blocco per controllare e aggiornare il prodotto
            {
                let prodotto = &mut self.prodotti[indice]; // Prestito mutabile qui

                if prodotto.quantita == 0 {
                    println!("Prodotto '{}' esaurito!", prodotto.nome);
                    // Non resettiamo il credito, l'utente potrebbe voler scegliere altro
                    return;
                }

                println!("Prodotto selezionato: {} - Prezzo: {:.2} EUR", prodotto.nome, (prodotto.prezzo as f32) / 100.0);

                if self.credito_inserito >= prodotto.prezzo {
                    prodotto.quantita -= 1;
                    resto_da_dare = self.credito_inserito - prodotto.prezzo;
                    nome_prodotto_acquistato = prodotto.nome.clone();
                    self.credito_inserito = 0; // Credito usato per l'acquisto
                    puo_acquistare = true;
                } else {
                    println!("Credito insufficiente. Hai {:.2} EUR, servono {:.2} EUR.",
                             (self.credito_inserito as f32) / 100.0,
                             (prodotto.prezzo as f32) / 100.0);
                    println!("Per favore, inserisci altre monete.");
                }
            } // Fine del prestito mutabile di 'prodotto'

            if puo_acquistare {
                println!("\n--- EROGAZIONE ---");
                println!("Prodotto '{}' erogato con successo!", nome_prodotto_acquistato);
                if resto_da_dare > 0 {
                    println!("Resto: {:.2} EUR", (resto_da_dare as f32) / 100.0);
                }
                println!("Grazie per l'acquisto!");
                println!("------------------");
            }

        } else {
            println!("ID prodotto non valido.");
        }
    }

    // Restituisce il credito inserito se l'utente annulla
    fn annulla_e_restituisci_credito(&mut self) {
        if self.credito_inserito > 0 {
            println!("Transazione annullata. Restituzione di {:.2} EUR.", (self.credito_inserito as f32) / 100.0);
            self.credito_inserito = 0;
        } else {
            println!("Nessun credito da restituire.");
        }
    }
}

// Funzione helper per leggere l'input dell'utente in modo pulito
fn leggi_input_utente(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Errore durante la lettura dell'input");
    input.trim().to_string()
}


fn main() {
    let mut distributore = Distributore::new();

    loop {
        distributore.mostra_menu();

        println!("\nCosa vuoi fare?");
        println!("1. Scegli prodotto");
        println!("2. Inserisci moneta");
        println!("3. Annulla e recupera credito");
        println!("4. Esci");

        let scelta_utente = leggi_input_utente("Opzione: ");

        match scelta_utente.as_str() {
            "1" => { // Scegli prodotto
                let id_str = leggi_input_utente("Inserisci l'ID del prodotto desiderato: \n [1] Coca Cola\n [2] Patatine\n [3] Acqua Minerale\n [4] Snack Cioccolato\n [5] Caffè Espresso\n");
                match id_str.parse::<u32>() {
                    Ok(id_prodotto) => {
                        distributore.acquista_prodotto(id_prodotto);
                    }
                    Err(_) => {
                        println!("ID prodotto non valido. Inserisci un numero.");
                    }
                }
            }
            "2" => { // Inserisci moneta
                distributore.inserisci_moneta();
            }
            "3" => { // Annulla e recupera credito
                distributore.annulla_e_restituisci_credito();
            }
            "4" => { // Esci
                if distributore.credito_inserito > 0 {
                    println!("Prima di uscire, recupero del credito residuo:");
                    distributore.annulla_e_restituisci_credito();
                }
                println!("Arrivederci!");
                break;
            }
            _ => {
                println!("Opzione non valida. Riprova.");
            }
        }
        leggi_input_utente("\nPremi INVIO per continuare..."); // Pausa per leggibilità
        clearscreen::clear().unwrap_or_else(|e| println!("Errore pulizia schermo: {}",e)); // Pulisce lo schermo
    }
}