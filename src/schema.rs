// @generated automatically by Diesel CLI.

diesel::table! {
    complementaries (id) {
        id -> Nullable<Integer>,
        word_id -> Integer,
        complementary_word -> Text,
        complementary_ipa -> Nullable<Text>,
        note -> Nullable<Text>,
    }
}

diesel::table! {
    correspondences (id) {
        id -> Nullable<Integer>,
        word_id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        name -> Text,
        rationale -> Nullable<Text>,
    }
}

diesel::table! {
    examples (id) {
        id -> Nullable<Integer>,
        word_id -> Integer,
        register -> Text,
        meikyo_sentence -> Text,
        translation -> Nullable<Text>,
    }
}

diesel::table! {
    root_polarities (id) {
        id -> Nullable<Integer>,
        word_id -> Integer,
        position -> Integer,
        consonant -> Text,
        agency -> Nullable<Text>,
        realm -> Nullable<Text>,
        element -> Nullable<Text>,
        quality -> Nullable<Text>,
    }
}

diesel::table! {
    word_contents (id) {
        id -> Nullable<Integer>,
        word_id -> Integer,
        section -> Text,
        content -> Text,
        language -> Nullable<Text>,
    }
}

diesel::table! {
    words (id) {
        id -> Nullable<Integer>,
        word -> Text,
        ipa -> Nullable<Text>,
        root -> Text,
        chakra -> Nullable<Text>,
        element -> Nullable<Text>,
        frequency -> Nullable<Integer>,
        grapheme -> Nullable<Text>,
        polarity -> Nullable<Text>,
        part_of_speech -> Nullable<Text>,
        created_at -> Nullable<Text>,
    }
}

diesel::joinable!(complementaries -> words (word_id));
diesel::joinable!(correspondences -> words (word_id));
diesel::joinable!(examples -> words (word_id));
diesel::joinable!(root_polarities -> words (word_id));
diesel::joinable!(word_contents -> words (word_id));

diesel::allow_tables_to_appear_in_same_query!(
    complementaries,
    correspondences,
    examples,
    root_polarities,
    word_contents,
    words,
);
