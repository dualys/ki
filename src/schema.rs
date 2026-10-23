// @generated automatically by Diesel CLI.

diesel::table! {
    complementaries (id) {
        id -> Int4,
        word_id -> Int4,
        complementary_word -> Text,
        complementary_ipa -> Nullable<Text>,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    concepts (id) {
        id -> Int4,
        english_word -> Text,
        implemented -> Nullable<Bool>,
        meikyo_root -> Nullable<Text>,
    }
}

diesel::table! {
    correspondences (id) {
        id -> Int4,
        word_id -> Int4,
        #[sql_name = "type"]
        type_ -> Text,
        name -> Text,
        rationale -> Nullable<Text>,
    }
}

diesel::table! {
    examples (id) {
        id -> Int4,
        word_id -> Int4,
        register -> Text,
        meikyo_sentence -> Text,
        translation -> Nullable<Text>,
    }
}

diesel::table! {
    root_polarities (id) {
        id -> Int4,
        word_id -> Int4,
        position -> Int4,
        consonant -> Text,
        agency -> Nullable<Text>,
        realm -> Nullable<Text>,
        element -> Nullable<Text>,
        quality -> Nullable<Text>,
    }
}

diesel::table! {
    word_contents (id) {
        id -> Int4,
        word_id -> Int4,
        section -> Text,
        content -> Text,
        language -> Nullable<Text>,
    }
}

diesel::table! {
    words (id) {
        id -> Int4,
        word -> Text,
        ipa -> Nullable<Text>,
        root -> Text,
        chakra -> Nullable<Text>,
        element -> Nullable<Text>,
        frequency -> Nullable<Int4>,
        grapheme -> Nullable<Text>,
        polarity -> Nullable<Text>,
        part_of_speech -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        definition -> Nullable<Text>,
        word_reversed -> Nullable<Text>,
        root_reversed -> Nullable<Text>,
        extensions -> Nullable<Jsonb>,
        ipa_reversed -> Nullable<Text>,
        hz -> Nullable<Int4>,
    }
}

diesel::joinable!(complementaries -> words (word_id));
diesel::joinable!(correspondences -> words (word_id));
diesel::joinable!(examples -> words (word_id));
diesel::joinable!(root_polarities -> words (word_id));
diesel::joinable!(word_contents -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    complementaries,
    concepts,
    correspondences,
    examples,
    root_polarities,
    word_contents,
    words,
);
