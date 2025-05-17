use std::io;

fn analyze_nutrition(
    energy: f32,
    fats: f32,
    saturated_fats: f32,
    carbs: f32,
    sugars: f32,
    fiber: f32,
    protein: f32,
    salt: f32,
) -> String {
    let mut analysis = String::new();

    if energy > 250.0 {
        analysis.push_str("Questo alimento è relativamente alto in calorie.");
    } else {
        analysis.push_str("Questo alimento è relativamente basso in calorie. ");
    }

    if fats > 20.0 {
        analysis.push_str("Questo alimento è alto in grassi. Considera di scegliere opzioni con un contenuto di grassi inferiore. ");
    } else if fats < 10.0 {
        analysis.push_str("Questo alimento è basso in grassi. ");
    } else {
        analysis.push_str("Questo alimento ha un contenuto di grassi moderato. ");
    }

    if saturated_fats > 3.0 {
        analysis.push_str("Questo alimento è alto in grassi saturi. Cerca di limitare i grassi saturi nella tua dieta. ");
    } else {
        analysis.push_str("Questo alimento ha un contenuto di grassi saturi moderato. ");
    }

    if carbs > 65.0 {
        analysis.push_str("Questo alimento è alto in carboidrati. ");
    } else if carbs < 50.0 {
        analysis.push_str("Questo alimento è basso in carboidrati. ");
    } else {
        analysis.push_str("Questo alimento ha un contenuto di carboidrati moderato. ");
    }

    if sugars > 25.0 {
        analysis.push_str("Questo alimento è alto in zuccheri. Sii consapevole del tuo consumo di zucchero. ");
    } else {
        analysis.push_str("Questo alimento ha un contenuto di zuccheri moderato. ");
    }

    if fiber < 6.0 {
        analysis.push_str("Questo alimento è relativamente basso in fibre. Punta a cibi con un contenuto di fibre più elevato. ");
    } else {
        analysis.push_str("Questo alimento è una buona fonte di fibre. ");
    }

    if protein < 7.0 {
        analysis.push_str("Questo alimento è basso in proteine. ");
    } else {
        analysis.push_str("Questo alimento è una buona fonte di proteine. ");
    }

    if salt > 0.5 {
        analysis.push_str("Questo alimento è alto in sale. Cerca di limitare l'assunzione di sale. ");
    } else {
        analysis.push_str("Questo alimento ha un contenuto di sale moderato. ");
    }

    if analysis.is_empty() {
        analysis.push_str("Questo alimento ha un profilo nutrizionale bilanciato.");
    }

    analysis
}

fn main() {
    println!("Inserisci i valori nutrizionali per 100g:");

    let mut input = String::new();

    println!("Energia (kcal): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let energy: f32 = input.trim().parse().expect("Input non valido");
    input.clear(); // Svuota la stringa di input

    println!("Grassi (g): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let fats: f32 = input.trim().parse().expect("Input non valido");
    input.clear();

    println!("Grassi saturi (g): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let saturated_fats: f32 = input.trim().parse().expect("Input non valido");
    input.clear();

    println!("Carboidrati (g): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let carbs: f32 = input.trim().parse().expect("Input non valido");
    input.clear();

    println!("Zuccheri (g): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let sugars: f32 = input.trim().parse().expect("Input non valido");
    input.clear();

    println!("Fibre (g): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let fiber: f32 = input.trim().parse().expect("Input non valido");
    input.clear();

    println!("Proteine (g): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let protein: f32 = input.trim().parse().expect("Input non valido");
    input.clear();

    println!("Sale (g): ");
    io::stdin().read_line(&mut input).expect("Errore di lettura");
    let salt: f32 = input.trim().parse().expect("Input non valido");

    let analysis = analyze_nutrition(
        energy, fats, saturated_fats, carbs, sugars, fiber, protein, salt,
    );
    println!("\nAnalisi:");
    println!("{}", analysis);
}

