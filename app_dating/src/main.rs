use std::collections::HashSet;

// Definiamo le possibili caratteristiche che un utente può avere.
// Deriviamo Debug per poter stampare facilmente;
// Clone per poterle copiare;
// PartialEq, Eq e Hash per poterle usare in un HashSet.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Caratteristica {
    Viaggi,
    MusicaRock,
    MusicaPop,
    Cucina,
    Sport,
    Lettura,
    Cinema,
    Arte,
    Tecnologia,
    Animali,
}

// Definiamo la struttura per un profilo utente.
#[derive(Debug, Clone)]
struct Utente {
    id: u32,
    nome: String,
    caratteristiche: HashSet<Caratteristica>,
}

impl Utente {
    // Costruttore per Utente
    fn new(id: u32, nome: &str, caratteristiche: HashSet<Caratteristica>) -> Self {
        Utente {
            id,
            nome: nome.to_string(),
            caratteristiche,
        }
    }
}

fn main() {
    // 1. Creiamo un array (Vec) di utenti con i loro profili e caratteristiche.
    let utenti = vec![
        Utente::new(
            1,
            "Alice",
            [
                Caratteristica::Viaggi,
                Caratteristica::MusicaRock,
                Caratteristica::Cucina,
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        Utente::new(
            2,
            "Bob",
            [
                Caratteristica::Sport,
                Caratteristica::MusicaRock,
                Caratteristica::Tecnologia,
                Caratteristica::Cinema,
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        Utente::new(
            3,
            "Charlie",
            [
                Caratteristica::Lettura,
                Caratteristica::Arte,
                Caratteristica::Animali,
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        Utente::new(
            4,
            "Diana",
            [
                Caratteristica::Viaggi,
                Caratteristica::Cucina,
                Caratteristica::MusicaPop,
                Caratteristica::Cinema,
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        Utente::new(
            5,
            "Eve",
            [
                Caratteristica::MusicaRock,
                Caratteristica::Tecnologia,
                Caratteristica::Sport,
                Caratteristica::Viaggi,
            ]
            .iter()
            .cloned()
            .collect(),
        ),
    ];

    println!("Simulazione App Dating di Facebook\n");

    // Soglia per considerare una "alta compatibilità"
    const ALTA_COMPATIBILITA_SOGLIA: usize = 2; // Ad esempio, 2 o più caratteristiche in comune

    // 2. & 3. Per ogni coppia di utenti, controlliamo le caratteristiche in comune.
    // Usiamo un ciclo nidificato, ma partiamo da j = i + 1 per evitare coppie duplicate
    // (es. Alice-Bob e Bob-Alice) e per evitare di confrontare un utente con se stesso.
    for i in 0..utenti.len() {
        for j in (i + 1)..utenti.len() {
            let utente1 = &utenti[i];
            let utente2 = &utenti[j];

            // Troviamo le caratteristiche in comune usando l'intersezione degli HashSet
            let caratteristiche_comuni: HashSet<_> = utente1
                .caratteristiche
                .intersection(&utente2.caratteristiche)
                .cloned() // cloniamo gli elementi perché intersection restituisce riferimenti
                .collect();

            if !caratteristiche_comuni.is_empty() {
                println!(
                    "Possibile Match trovato tra {} (ID: {}) e {} (ID: {}):",
                    utente1.nome, utente1.id, utente2.nome, utente2.id
                );
                print!("  Caratteristiche in comune: ");
                for (idx, caratteristica) in caratteristiche_comuni.iter().enumerate() {
                    if idx > 0 {
                        print!(", ");
                    }
                    print!("{:?}", caratteristica);
                }
                println!("."); // Nuova riga dopo le caratteristiche

                // 4. Se ci sono più caratteristiche in comune, allora alta compatibilità.
                if caratteristiche_comuni.len() >= ALTA_COMPATIBILITA_SOGLIA {
                    println!(
                        "  Compatibilità: Alta ({})",
                        caratteristiche_comuni.len()
                    );
                } else {
                    println!(
                        "  Compatibilità: Media ({})",
                        caratteristiche_comuni.len()
                    );
                }
                println!("---"); // Separatore
            }
        }
    }
    println!("\nFine della simulazione.");
}