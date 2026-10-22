use inquire::{Confirm, InquireError, Select, Text};
const QUESTIONS: &[(&str, &str)] = &[
    (
        "Does it anchor, stabilize, or provide a foundation?",
        "Root",
    ),
    ("Does it flow, stir emotions, create, or desire?", "Sacral"),
    ("Does it act, will, transform, or decide?", "Solar Plexus"),
    ("Does it connect, love, breathe, or relate?", "Heart"),
    ("Does it express, speak, or resonate?", "Throat"),
    (
        "Does it see, discern, understand, or illuminate?",
        "Third Eye",
    ),
    (
        "Does it unite, transcend, or relate to silence and the absolute?",
        "Crown",
    ),
];

pub fn add_element_prefix(word: &str, polarities: &[(String, String, String, String)]) -> String {
    if polarities.is_empty() {
        return word.to_string();
    }

    // On prend l'élément de la première consonne comme dominant
    let dominant_element = &polarities[0].2;

    let prefix = match dominant_element.as_str() {
        "Earth" => "ta-",
        "Water" => "la-",
        "Fire" => "ka-",
        "Air" => "ya-",
        "Ether" => "ha-",
        "Light" => "sha-",
        "Cosmos" => "a-",
        _ => "",
    };

    if prefix.is_empty() {
        word.to_string()
    } else {
        format!("{}'{word}", prefix.trim_end_matches('-'))
    }
}
pub fn generate_definition(polarities: &[(String, String, String, String)]) -> String {
    if polarities.len() != 3 {
        return "Definition not available.".to_string();
    }

    let descriptions: Vec<String> = polarities
        .iter()
        .map(|(agency, realm, element, quality)| {
            let action = if agency == "Active" {
                "emits"
            } else {
                "receives"
            };
            let realm_word = if realm == "Celestial" {
                "spiritual"
            } else {
                "material"
            };
            let quality_word = if quality == "Force" {
                "with force"
            } else {
                "with grace"
            };

            format!(
                "{} {} {} energy {}",
                action,
                realm_word,
                element.to_lowercase(),
                quality_word
            )
        })
        .collect();

    format!(
        "A principle that {}, {}, and {}.",
        descriptions[0], descriptions[1], descriptions[2]
    )
}
// Step 2 — Choose 3 consonants linked to the chakra
pub fn choose_consonants(chakra: &str) -> Result<Vec<String>, InquireError> {
    let suggestions: Vec<&str> = match chakra {
        "Root" => vec!["T", "P", "D", "B", "Ṭ", "Ḍ", "M", "N"],
        "Sacral" => vec!["L", "R", "V", "F", "M", "N", "W", "Y"],
        "Solar Plexus" => vec!["K", "G", "S", "Z", "C", "J", "T", "D"],
        "Heart" => vec!["Y", "W", "M", "N", "L", "R", "H", "'"],
        "Throat" => vec!["H", "'", "Th", "Dh", "S", "Sh", "Zh", "Y"],
        "Third Eye" => vec!["Sh", "Zh", "Ṇ", "Ñ", "Y", "W", "L", "R"],
        "Crown" => vec!["A", "Ā", "M", "N", "H", "'", "Sh", "Zh"],
        _ => vec!["T", "L", "K", "Y", "Sh", "H"],
    };

    let mut selected: Vec<&str> = Vec::new();

    for _ in 0..3 {
        selected.push(Select::new("Select a consonant", suggestions.to_vec()).prompt()?);
    }

    if selected.len() >= 3 {
        Ok(selected
            .into_iter()
            .take(3)
            .map(|s| s.to_string())
            .collect())
    } else {
        let manual = Text::new("Enter 3 consonants separated by spaces:").prompt()?;
        Ok(manual.split_whitespace().map(|s| s.to_string()).collect())
    }
}

// Step 3 — Assign the 4 polarities
pub fn assign_polarities(
    consonants: &[String],
) -> Result<Vec<(String, String, String, String)>, InquireError> {
    let mut result = Vec::new();

    for (i, c) in consonants.iter().enumerate() {
        println!("\nConsonant {} : {}", i + 1, c);

        let agency = Select::new(
            "Agency (Does it emit or receive?):",
            vec!["Active", "Passive"],
        )
        .prompt()?;

        let realm = Select::new(
            "Realm (Spiritual principle or material manifestation?):",
            vec!["Celestial", "Terrestrial"],
        )
        .prompt()?;

        let element = Select::new(
            "Element (What energy does it carry?):",
            vec!["Earth", "Water", "Fire", "Air", "Ether", "Light", "Cosmos"],
        )
        .prompt()?;

        let quality = Select::new(
            "Quality (Direct power or subtle influence?):",
            vec!["Force", "Grace"],
        )
        .prompt()?;

        result.push((
            agency.to_string(),
            realm.to_string(),
            element.to_string(),
            quality.to_string(),
        ));
    }

    Ok(result)
}
// Étape 2 — Choisir 3 consonnes liées au chakra
pub fn determine_chakra() -> Result<String, InquireError> {
    for (question, chakra) in QUESTIONS {
        let answer = Confirm::new(question).with_default(false).prompt()?;
        if answer {
            return Ok(chakra.to_string());
        }
    }
    // Si aucune réponse positive
    Ok("Couronne".to_string()) // ou une valeur par défaut
}

pub fn generate_all_forms(consonants: &[String]) -> Vec<(String, String)> {
    if consonants.len() != 3 {
        return vec![];
    }

    let c1 = &consonants[0];
    let c2 = &consonants[1];
    let c3 = &consonants[2];

    let verb = format!("{c1}a{c2}a{c3}a").to_lowercase();
    let noun = format!("{c1}i{c2}ā{c3}").to_lowercase();

    let mut forms = vec![
        ("Verb".to_string(), verb.clone()),
        ("Noun".to_string(), noun.clone()),
        ("Verbe inversé".to_string(), verb.chars().rev().collect()),
        ("Noun inversé".to_string(), noun.chars().rev().collect()),
    ];

    // Formes supplémentaires
    forms.push(("Verb + -yū".to_string(), format!("{verb}-yū")));
    forms.push(("Noun + -ūm".to_string(), format!("{noun}-ūm")));
    forms.push((
        "Verb with apostrophe".to_string(),
        format!("{c1}a'{c2}a{c3}a").to_lowercase(),
    ));
    forms.push((
        "Noun with apostrophe".to_string(),
        format!("{c1}i'{c2}ā{c3}").to_lowercase(),
    ));
    // Dans generate_all_forms, après les formes existantes :

    forms.push(("Verb + me-".to_string(), format!("me-{}", verb)));
    forms.push(("Noun + me-".to_string(), format!("me-{}", noun)));
    forms.push(("Verb + aś-".to_string(), format!("aś-{}", verb)));
    forms.push(("Noun + aś-".to_string(), format!("aś-{}", noun)));

    // Combinaisons
    forms.push(("me- + -yū".to_string(), format!("me-{}-yū", verb)));
    forms.push(("aś- + -ūm".to_string(), format!("aś-{}-ūm", noun)));
    forms
}

pub fn add_word_interactive() -> Result<(), InquireError> {
    let chakra = determine_chakra()?;
    let consonants = choose_consonants(&chakra)?;
    let polarities = assign_polarities(&consonants)?;

    let forms = generate_all_forms(&consonants);

    for (label, form) in &forms {
        println!("{label} : {form}");
    }
    let definition = generate_definition(&polarities);
    println!("Définition :\n\n\t{definition}");
    Ok(())
}
