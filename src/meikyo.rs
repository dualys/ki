use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Integer;
use diesel::sql_types::Text;
use inquire::{Confirm, InquireError, MultiSelect, Select};
use std::env::var;
use std::io::Write;
use tabled::{builder::Builder, settings::Style};
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

// ============================================================
// 1. FILL NULL COLUMNS
// ============================================================

#[derive(QueryableByName, Debug)]
struct WordRow {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    word: String,
    #[diesel(sql_type = Text)]
    chakra: String,
    #[diesel(sql_type = Text)]
    word_reversed: String, // plus d'Option
}

pub fn fill_null_columns(conn: &mut PgConnection) -> Result<(), diesel::result::Error> {
    let rows: Vec<WordRow> = sql_query(
        "SELECT id, word, chakra, COALESCE(word_reversed, '') AS word_reversed \
         FROM words WHERE element IS NULL OR ipa IS NULL",
    )
    .load(conn)?;
    let chakra_to_hz = [
        ("Root", 396),
        ("Sacral", 417),
        ("Solar Plexus", 528),
        ("Heart", 639),
        ("Throat", 741),
        ("Third Eye", 852),
        ("Crown", 963),
    ];
    let chakra_to_element = [
        ("Root", "Earth"),
        ("Sacral", "Water"),
        ("Solar Plexus", "Fire"),
        ("Heart", "Air"),
        ("Throat", "Ether"),
        ("Third Eye", "Light"),
        ("Crown", "Cosmos"),
    ];

    for row in rows {
        let element = chakra_to_element
            .iter()
            .find(|(c, _)| *c == row.chakra)
            .map(|(_, e)| *e)
            .unwrap_or("Ether");

        let ipa = to_ipa(&row.word);
        let ipa_reversed = if row.word_reversed.is_empty() {
            String::new()
        } else {
            to_ipa(&row.word_reversed)
        };

        sql_query(
            "UPDATE words SET element = $1, ipa = $2, ipa_reversed = $3, \
             part_of_speech = 'n.', polarity = 'neutral' WHERE id = $4",
        )
        .bind::<Text, _>(element)
        .bind::<Text, _>(&ipa)
        .bind::<Text, _>(&ipa_reversed)
        .bind::<Integer, _>(row.id)
        .execute(conn)?;
        let hz = chakra_to_hz
            .iter()
            .find(|(c, _)| *c == row.chakra)
            .map(|(_, h)| *h)
            .unwrap_or(396);

        // Dans le UPDATE
        sql_query(
            "UPDATE words SET element = $1, ipa = $2, ipa_reversed = $3, \
     part_of_speech = 'n.', polarity = 'neutral', frequency = $5 WHERE id = $4",
        )
        .bind::<Text, _>(element)
        .bind::<Text, _>(&ipa)
        .bind::<Text, _>(&ipa_reversed)
        .bind::<Integer, _>(row.id)
        .bind::<Integer, _>(hz)
        .execute(conn)?;
    }

    Ok(())
}

// ============================================================
// 2. EXPORT DICTIONARY
// ============================================================

#[derive(QueryableByName, Debug)]
struct ExportRow {
    #[diesel(sql_type = Text)]
    word: String,
    #[diesel(sql_type = Text)]
    ipa: String,
    #[diesel(sql_type = Text)]
    root: String,
    #[diesel(sql_type = Text)]
    chakra: String,
    #[diesel(sql_type = Text)]
    definition: String,
}

pub fn export_dictionary(conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
    let words: Vec<ExportRow> = sql_query(
        "SELECT word, COALESCE(ipa, '') AS ipa, root, chakra, definition \
         FROM meikyo_core_lexicon ORDER BY word ASC",
    )
    .load(conn)?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("dictionary.typ")?;

    writeln!(file, "#set page(margin: 1.5cm)")?;
    writeln!(file, "#set text(font: \"New Computer Modern\", size: 10pt)")?;
    writeln!(file, "#set heading(numbering: \"1.\")")?;
    writeln!(file, "= Meikyō Dictionary")?;
    writeln!(file)?;

    for w in &words {
        writeln!(file, "== {} [{}] {} {}", w.word, w.ipa, w.root, w.chakra)?;
        writeln!(file, "{}", w.definition)?;
        writeln!(file)?;
    }

    let status = std::process::Command::new("typst")
        .args(["compile", "dictionary.typ", "meikyo.pdf"])
        .status()?;

    if status.success() {
        println!("PDF généré : meikyo.pdf ({} mots)", words.len());
    } else {
        eprintln!("Erreur Typst");
    }

    Ok(())
}

pub fn to_ipa(word: &str) -> String {
    let mut result = word.to_lowercase();

    let replacements = [
        ("sh", "ʃ"),
        ("zh", "ʒ"),
        ("th", "θ"),
        ("dh", "ð"),
        ("ch", "tʃ"),
        ("ñ", "ɲ"),
        ("ṇ", "ɳ"),
        ("ṭ", "ʈ"),
        ("ḍ", "ɖ"),
        ("ā", "aː"),
        ("ī", "iː"),
        ("ū", "uː"),
        ("ē", "eː"),
        ("ō", "oː"),
        ("'", "ʔ"),
        ("aś", "aʃ"),
        ("-yū", "-juː"),
        ("-ūm", "-uːm"),
        ("-en", "-en"),
        ("-or", "-or"),
    ];

    for (from, to) in replacements {
        result = result.replace(from, to);
    }
    result.replace("--", "-")
}

// ============================================================
// 3. IMPORT WORDLIST
// ============================================================

pub fn import_wordlist(
    conn: &mut PgConnection,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::concepts::dsl::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let word = line?;
        let word = word.trim();
        if word.is_empty() || word.len() < 2 {
            continue;
        }

        diesel::insert_into(concepts)
            .values(english_word.eq(word))
            .on_conflict(english_word)
            .do_nothing()
            .execute(conn)?;
    }

    println!("Import terminé depuis {}", path);
    Ok(())
}

pub fn suggest_words(
    conn: &mut PgConnection,
    letter: &str,
    limit: i64,
) -> Result<Vec<String>, diesel::result::Error> {
    use crate::schema::concepts::dsl::*;
    // On construit un filtre simple (contient au moins une des lettres)
    let mut query = concepts.filter(implemented.eq(false)).into_boxed();

    if !letter.is_empty() {
        let pattern = format!("%{letter}%");
        query = query.filter(english_word.ilike(pattern));
    }
    let results: Vec<String> = query.select(english_word).limit(limit).load(conn)?;
    Ok(results)
}

pub fn interactive_generation_loop(
    conn: &mut PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::concepts::dsl::*;
    loop {
        // 1. Sélection des lettres
        let alphabet = [
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];

        let selected_letters =
            MultiSelect::new("Select starting letters:", alphabet.to_vec()).prompt()?;

        if selected_letters.is_empty() {
            println!("No letters selected.");
            continue;
        }

        // 2. Pour chaque lettre, proposer des mots
        for letter in &selected_letters {
            let suggestions = suggest_words(conn, letter, 5)?;

            if suggestions.is_empty() {
                println!("No words found starting with '{letter}'.");
                continue;
            }

            let chosen_words =
                MultiSelect::new("Select words to implement:", suggestions).prompt()?;

            for w in &chosen_words {
                if let Err(e) = add_word_interactive() {
                    eprintln!("Error during generation: {}", e);
                    continue;
                }
                diesel::update(concepts.filter(english_word.eq(w)))
                    .set(implemented.eq(true))
                    .execute(conn)?;
            }
        }

        // Demander si on continue
        let continue_loop = Confirm::new("Continue with other letters?")
            .with_default(true)
            .prompt()?;

        if !continue_loop {
            break;
        }
    }

    Ok(())
}

pub fn save_word_and_forms(
    conn: &mut PgConnection,
    root: &str,
    chakra: &str,
    consonants: &[String],
    polarities: &[(String, String, String, String)],
    forms: &[(String, String)],
    definition: &str,
) -> Result<(), diesel::result::Error> {
    use crate::schema::{examples, root_polarities, word_contents, words};

    let main_form = forms.first().map(|f| f.1.as_str()).unwrap_or(root);
    let main_ipa = to_ipa(main_form);

    // Insertion + récupération de l'id (PostgreSQL supporte RETURNING)
    let word_id: i32 = diesel::insert_into(words::table)
        .values((
            words::word.eq(main_form),
            words::ipa.eq(&main_ipa),
            words::root.eq(root),
            words::chakra.eq(chakra),
        ))
        .returning(words::id)
        .get_result(conn)?;

    // Polarités
    for (i, (agency, realm, element, quality)) in polarities.iter().enumerate() {
        let consonant = consonants.get(i).map(|s| s.as_str()).unwrap_or("");

        diesel::insert_into(root_polarities::table)
            .values((
                root_polarities::word_id.eq(word_id),
                root_polarities::position.eq((i + 1) as i32),
                root_polarities::consonant.eq(consonant),
                root_polarities::agency.eq(agency),
                root_polarities::realm.eq(realm),
                root_polarities::element.eq(element),
                root_polarities::quality.eq(quality),
            ))
            .execute(conn)?;
    }

    // Définition
    diesel::insert_into(word_contents::table)
        .values((
            word_contents::word_id.eq(word_id),
            word_contents::section.eq("Definition"),
            word_contents::content.eq(definition),
            word_contents::language.eq("en"),
        ))
        .execute(conn)?;

    // Formes + IPA
    for (label, form) in forms {
        let ipa = to_ipa(form);

        diesel::insert_into(examples::table)
            .values((
                examples::word_id.eq(word_id),
                examples::register.eq(label),
                examples::meikyo_sentence.eq(form),
                examples::translation.eq(&ipa),
            ))
            .execute(conn)?;
    }
    Ok(())
}

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
        Ok(vec![])
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
pub fn generate_all_affixed_forms(consonants: &[String]) -> Vec<(String, String)> {
    if consonants.len() != 3 {
        return vec![];
    }

    let c1 = &consonants[0];
    let c2 = &consonants[1];
    let c3 = &consonants[2];

    let verb = format!("{c1}a{c2}a{c3}a").to_lowercase();
    let noun = format!("{c1}i{c2}ā{c3}").to_lowercase();

    let prefixes = [
        "", "me-", "aś-", "ta-", "la-", "ka-", "ya-", "ha-", "sha-", "a-",
    ];
    let suffixes = ["", "-yū", "-ūm", "-en", "-or"];

    let mut forms = Vec::new();

    for prefix in prefixes {
        for suffix in suffixes {
            let v_form = format!("{prefix}{verb}{suffix}");
            let n_form = format!("{prefix}{noun}{suffix}");

            let v_label = format!(
                "Verb {}{}",
                if prefix.is_empty() { "" } else { prefix },
                if suffix.is_empty() { "" } else { suffix }
            );
            let n_label = format!(
                "Noun {}{}",
                if prefix.is_empty() { "" } else { prefix },
                if suffix.is_empty() { "" } else { suffix }
            );

            forms.push((v_label, v_form));
            forms.push((n_label, n_form));
        }
    }

    // Ajout des inversions de base
    forms.push(("Verb inverted".to_string(), verb.chars().rev().collect()));
    forms.push(("Noun inverted".to_string(), noun.chars().rev().collect()));

    forms
}
pub fn add_word_interactive() -> Result<(), InquireError> {
    let chakra = determine_chakra()?;
    let consonants = choose_consonants(&chakra)?;
    let polarities = assign_polarities(&consonants)?;

    let forms = generate_all_affixed_forms(&consonants);
    let mut table = Builder::new();
    let definition = generate_definition(&polarities);
    table.push_record(["Form", "Definition"]);
    for (label, form) in &forms {
        let adapted = adapt_definition(&definition, label);
        table.push_record([form, &adapted]);
    }
    let mut t = table.build();
    println!("{}", t.with(Style::modern()).to_string());

    let mut conn =
        PgConnection::establish(var("DATABASE_URL").expect("DATABASE_URL").as_str()).expect("");

    save_word_and_forms(
        &mut conn,
        &consonants.join("-"),
        &chakra,
        &consonants, // ← passé
        &polarities,
        &forms,
        &definition,
    )
    .expect("failed to save");
    Ok(())
}

pub fn adapt_definition(base_definition: &str, form_label: &str) -> String {
    match form_label {
        "Verb" => format!("Process: {}", base_definition),
        "Noun" => format!("State: {}", base_definition),
        "Verbe inversé" | "Noun inversé" => {
            format!("Complementary: {}", base_definition)
        }
        label if label.contains("me-") => {
            format!("In relation / mirror: {}", base_definition)
        }
        label if label.contains("aś-") => {
            format!("Inner invocation: {}", base_definition)
        }
        label if label.contains("-yū") => {
            format!("In elevation: {}", base_definition)
        }
        label if label.contains("-ūm") => {
            format!("In rooting: {}", base_definition)
        }
        label if label.contains("apostrophe") => {
            format!("With breath: {}", base_definition)
        }
        _ => base_definition.to_string(),
    }
}
pub fn generate_definition(polarities: &[(String, String, String, String)]) -> String {
    if polarities.len() != 3 {
        return "Definition not available.".to_string();
    }

    let mut phrases = Vec::new();

    for (agency, realm, element, quality) in polarities {
        let action = if agency == "Active" {
            "manifests"
        } else {
            "receives"
        };

        let realm_adj = if realm == "Celestial" {
            "spiritual"
        } else {
            "earthly"
        };

        let quality_adv = if quality == "Force" {
            "strongly"
        } else {
            "gently"
        };

        phrases.push(format!(
            "{} {} {} {} energy",
            action,
            quality_adv,
            realm_adj,
            element.to_lowercase()
        ));
    }

    format!(
        "A principle that {}, {}, and {}.",
        phrases[0], phrases[1], phrases[2]
    )
}
