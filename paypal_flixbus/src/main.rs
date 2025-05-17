use std::io::{self, Write};
use uuid::Uuid;
use chrono::{Utc, DateTime};

// --- Strutture Dati ---

#[derive(Debug, Clone)]
enum TicketStatus {
    Confirmed,
    Refunded, 
    Cancelled,
}

#[derive(Debug, Clone)]
struct Trip {
    id: String,
    origin: String,
    destination: String,
    departure_date_time: String, 
    price_cents: u32, 
    available_seats: u32,
}

#[derive(Debug, Clone)]
struct PassengerDetails {
    name: String,
    email: String, 
    phone: String,
}

#[derive(Debug, Clone)]
struct PurchasedTicket {
    ticket_id: String,
    trip_id: String,
    passenger_name: String,
    passenger_email: String,
    purchase_date_time: DateTime<Utc>,
    status: TicketStatus,
    payment_transaction_id: String,
    price_paid_cents: u32,
}

struct FlixbusSystem {
    trips: Vec<Trip>,
    purchased_tickets: Vec<PurchasedTicket>,
}

impl FlixbusSystem {
    fn new() -> Self {
        FlixbusSystem {
            trips: vec![
                Trip {
                    id: Uuid::new_v4().to_string(),
                    origin: "Milano".to_string(),
                    destination: "Roma".to_string(),
                    departure_date_time: "2025-07-15T09:00:00Z".to_string(),
                    price_cents: 3500, 
                    available_seats: 50,
                },
                Trip {
                    id: Uuid::new_v4().to_string(),
                    origin: "Roma".to_string(),
                    destination: "Napoli".to_string(),
                    departure_date_time: "2025-07-16T14:30:00Z".to_string(),
                    price_cents: 1990, 
                    available_seats: 30,
                },
                Trip {
                    id: Uuid::new_v4().to_string(),
                    origin: "Milano".to_string(),
                    destination: "Roma".to_string(), 
                    departure_date_time: "2025-07-15T15:00:00Z".to_string(),
                    price_cents: 3200, 
                    available_seats: 5,
                },
            ],
            purchased_tickets: Vec::new(),
        }
    }

    fn display_available_trips(&self) {
        println!("\n--- Viaggi Disponibili ---");
        if self.trips.is_empty() {
            println!("Nessun viaggio disponibile al momento.");
            return;
        }
        for (index, trip) in self.trips.iter().enumerate() {
            if trip.available_seats > 0 {
                println!(
                    "{}. ID: {} ({} -> {}) - Partenza: {} - Prezzo: {:.2} EUR - Posti: {}",
                    index + 1,
                    trip.id.chars().take(8).collect::<String>(),
                    trip.origin,
                    trip.destination,
                    trip.departure_date_time,
                    trip.price_cents as f32 / 100.0,
                    trip.available_seats
                );
            }
        }
        println!("--------------------------");
    }

    fn get_trip_by_index(&self, index: usize) -> Option<&Trip> {
        self.trips.get(index)
    }

    fn get_mut_trip_by_id(&mut self, trip_id: &str) -> Option<&mut Trip> {
        self.trips.iter_mut().find(|t| t.id == trip_id)
    }
    
    fn get_passenger_details_from_user(&self) -> PassengerDetails {
        println!("\n--- Dati del Passeggero ---");
        let name = Self::read_line_from_user("Nome completo del passeggero: ");
        let email = Self::read_line_from_user("Email del passeggero (usata per controllo duplicati): ");
        let phone = Self::read_line_from_user("Numero di telefono del passeggero: ");
        PassengerDetails { name, email, phone }
    }

    fn simulate_secure_payment(&self, amount_cents: u32) -> Result<String, String> {
        println!("\n--- Simulazione Servizio di Pagamento Sicuro (SPS) ---");
        println!("Importo da addebitare: {:.2} EUR", amount_cents as f32 / 100.0);
        loop {
            let choice = Self::read_line_from_user("Simulare pagamento riuscito? (s/n): ");
            match choice.to_lowercase().as_str() {
                "s" | "si" => {
                    let transaction_id = format!("SPS_TX_{}", Uuid::new_v4().to_string().chars().take(12).collect::<String>());
                    println!("Pagamento approvato dal SPS. ID Transazione: {}", transaction_id);
                    println!("----------------------------------------------------");
                    return Ok(transaction_id);
                }
                "n" | "no" => {
                    println!("Pagamento fallito o annullato dal SPS.");
                    println!("----------------------------------------------------");
                    return Err("Pagamento fallito presso SPS.".to_string());
                }
                _ => println!("Input non valido. Rispondere 's' o 'n'."),
            }
        }
    }

    fn check_for_duplicate_ticket(&self, trip_id_to_check: &str, passenger_email_to_check: &str) -> Option<&PurchasedTicket> {
        self.purchased_tickets.iter().find(|ticket| {
            ticket.trip_id == trip_id_to_check &&
            ticket.passenger_email.to_lowercase() == passenger_email_to_check.to_lowercase() &&
            matches!(ticket.status, TicketStatus::Confirmed)
        })
    }

    fn simulate_refund_via_sps(&self, payment_transaction_id: &str, amount_cents: u32) -> bool {
        println!("\n--- Simulazione Rimborso tramite SPS ---");
        println!(
            "Richiesta di rimborso per transazione SPS ID: {} di {:.2} EUR",
            payment_transaction_id,
            amount_cents as f32 / 100.0
        );
        println!("Rimborso processato con successo dal SPS (simulato).");
        println!("--------------------------------------");
        true 
    }

fn issue_new_ticket(
    &mut self,
    trip_id: String, // 'trip_id' viene mosso qui quando la funzione è chiamata
    passenger_details: PassengerDetails, // 'passenger_details' viene mossa qui
    payment_transaction_id: String, // 'payment_transaction_id' viene mossa qui
    price_paid_cents: u32,
) -> Result<String, String> {
    if let Some(trip) = self.get_mut_trip_by_id(&trip_id) { // Cerca il viaggio per ID
        if trip.available_seats > 0 {
            trip.available_seats -= 1; // Decrementa i posti
            let new_ticket_id_str = format!("FLXTKT_{}", Uuid::new_v4().to_string().chars().take(8).collect::<String>().to_uppercase());
            
            // Creazione del nuovo biglietto. Qui avviene il "move" dei campi String.
            let new_ticket = PurchasedTicket {
                ticket_id: new_ticket_id_str.clone(), // Cloniamo l'ID perché lo restituiremo
                trip_id, // trip_id (String) viene mosso qui
                passenger_name: passenger_details.name, // passenger_details.name (String) viene MOSSO qui
                passenger_email: passenger_details.email, // passenger_details.email (String) viene MOSSO qui
                purchase_date_time: Utc::now(),
                status: TicketStatus::Confirmed,
                payment_transaction_id, // payment_transaction_id (String) viene mosso qui
                price_paid_cents,
            };

            // A questo punto, passenger_details.name e .email non sono più accessibili
            // perché sono stati spostati in new_ticket.
            // Dobbiamo usare i campi di `new_ticket` per la stampa.

            println!("\n--- Biglietto Emesso con Successo ---");
            println!("ID Biglietto: {}", new_ticket.ticket_id); // Usa il campo da new_ticket
            println!("Passeggero: {}", new_ticket.passenger_name); // Usa il campo da new_ticket
            println!("Email: {}", new_ticket.passenger_email);     // Usa il campo da new_ticket
            println!("Viaggio ID: {}", new_ticket.trip_id.chars().take(8).collect::<String>()); // Usa il campo da new_ticket
            println!("Pagamento ID: {}", new_ticket.payment_transaction_id); // Usa il campo da new_ticket
            println!("Stato: {:?}", new_ticket.status); // È buona norma usare {:?} per gli enum
            println!("Prezzo Pagato: {:.2} EUR", new_ticket.price_paid_cents as f32 / 100.0);
            println!("Data Acquisto: {}", new_ticket.purchase_date_time.format("%Y-%m-%d %H:%M:%S UTC"));
            println!("-------------------------------------");
            
            self.purchased_tickets.push(new_ticket); // new_ticket viene mosso nel vettore

            Ok(new_ticket_id_str) // Restituisce l'ID clonato
        } else {
            Err("Posti esauriti poco prima della conferma finale.".to_string())
        }
    } else {
        // Questo errore non dovrebbe accadere se selected_trip_clone.id è sempre valido,
        // ma è una buona protezione.
        Err("Viaggio non trovato per l'emissione del biglietto (errore interno).".to_string())
    }
}

    fn process_purchase_attempt(&mut self) {
        self.display_available_trips();
        let choice_str = Self::read_line_from_user("Scegli il numero del viaggio da acquistare (o '0' per annullare): ");
        let choice_idx = match choice_str.parse::<usize>() {
            Ok(0) => {
                println!("Acquisto annullato.");
                return;
            }
            Ok(idx) if idx > 0 && idx <= self.trips.len() => idx -1,
            _ => {
                println!("Scelta non valida.");
                return;
            }
        };

        let selected_trip_clone: Trip; 
        if let Some(trip) = self.get_trip_by_index(choice_idx) {
             if trip.available_seats == 0 {
                println!("Spiacenti, il viaggio selezionato è esaurito.");
                return;
            }
            selected_trip_clone = trip.clone();
        } else {
            println!("Selezione viaggio non valida.");
            return;
        }


        println!("\nHai selezionato il viaggio: {} -> {} ({:.2} EUR)",
            selected_trip_clone.origin, selected_trip_clone.destination, selected_trip_clone.price_cents as f32 / 100.0);

        let passenger = self.get_passenger_details_from_user();

        println!("\n--- Riepilogo Acquisto ---");
        println!("Viaggio: {} -> {} il {}", selected_trip_clone.origin, selected_trip_clone.destination, selected_trip_clone.departure_date_time);
        println!("Passeggero: {}, Email: {}", passenger.name, passenger.email);
        println!("Prezzo Totale: {:.2} EUR", selected_trip_clone.price_cents as f32 / 100.0);
        
        let confirm_payment = Self::read_line_from_user("Confermi di voler procedere al pagamento? (s/n): ");
        if !(confirm_payment.to_lowercase() == "s" || confirm_payment.to_lowercase() == "si") {
            println!("Acquisto annullato dall'utente prima del pagamento.");
            return;
        }

        match self.simulate_secure_payment(selected_trip_clone.price_cents) {
            Ok(payment_id) => {
                if let Some(original_ticket) = self.check_for_duplicate_ticket(&selected_trip_clone.id, &passenger.email) {
                    println!("\n--- RILEVATO ACQUISTO DUPLICATO ---");
                    println!("Attenzione! Risulta già un biglietto confermato per:");
                    println!("  Passeggero Email: {}", passenger.email);
                    println!("  Viaggio ID: {}", selected_trip_clone.id.chars().take(8).collect::<String>());
                    println!("  Biglietto Originale ID: {}", original_ticket.ticket_id);
                    println!("Il tuo biglietto originale è ancora valido.");
                    println!("\nQuesto nuovo tentativo di acquisto (Transazione SPS: {}) verrà annullato e rimborsato.", payment_id);
                    
                    if self.simulate_refund_via_sps(&payment_id, selected_trip_clone.price_cents) {
                        println!("Il pagamento per questo acquisto duplicato è stato rimborsato.");
                    } else {
                        println!("ATTENZIONE: Il rimborso automatico per l'acquisto duplicato è fallito (simulato). Contattare l'assistenza.");
                    }
                    println!("------------------------------------");
                } else {
                    match self.issue_new_ticket(
                        selected_trip_clone.id.clone(), 
                        passenger,
                        payment_id.clone(),
                        selected_trip_clone.price_cents,
                    ) {
                        Ok(_new_ticket_id) => {
                            // Messaggio già stampato
                        }
                        Err(e) => {
                            println!("Errore critico durante l'emissione del biglietto: {}", e);
                            println!("Tentativo di rimborso per la transazione: {}", payment_id);
                            if self.simulate_refund_via_sps(&payment_id, selected_trip_clone.price_cents) {
                                println!("Pagamento rimborsato a seguito dell'errore di emissione.");
                            } else {
                                println!("ATTENZIONE CRITICA: Errore emissione E fallimento rimborso (simulato). Contattare l'assistenza IMMEDIATAMENTE.");
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Acquisto interrotto: {}", e);
            }
        }
    }
    
    fn view_my_tickets(&self) {
        let email_to_search = Self::read_line_from_user("Inserisci la tua email per visualizzare i biglietti: ");
        println!("\n--- I Tuoi Biglietti Acquistati (per {}) ---", email_to_search);
        let mut found_any = false;
        for ticket in &self.purchased_tickets {
            if ticket.passenger_email.to_lowercase() == email_to_search.to_lowercase() {
                println!(
                    "ID Biglietto: {}, Viaggio ID: {}, Passeggero: {}, Stato: {:?}, Data Acquisto: {}",
                    ticket.ticket_id,
                    ticket.trip_id.chars().take(8).collect::<String>(),
                    ticket.passenger_name,
                    ticket.status,
                    ticket.purchase_date_time.format("%Y-%m-%d %H:%M:%S UTC")
                );
                found_any = true;
            }
        }
        if !found_any {
            println!("Nessun biglietto trovato per l'email: {}", email_to_search);
        }
        println!("---------------------------------------------");
    }

    fn read_line_from_user(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Errore durante la lettura dell'input.");
        input.trim().to_string()
    }
}


fn main() {
    let mut flixbus_system = FlixbusSystem::new();

    println!("*** Benvenuto nel Sistema di Prenotazione Flixbus (Simulazione) ***");

    loop {
        println!("\nMenu Principale:");
        println!("1. Acquista un biglietto");
        println!("2. Visualizza i miei biglietti (tramite email)");
        println!("3. Esci");

        let choice = FlixbusSystem::read_line_from_user("Scegli un'opzione: "); // Chiamata corretta per funzione associata

        match choice.as_str() {
            "1" => flixbus_system.process_purchase_attempt(),
            "2" => flixbus_system.view_my_tickets(),
            "3" => {
                println!("Grazie per aver usato la simulazione Flixbus. Arrivederci!");
                break;
            }
            _ => println!("Opzione non valida. Riprova."),
        }
        FlixbusSystem::read_line_from_user("\nPremi INVIO per tornare al menu principale..."); // Chiamata corretta
    }
}