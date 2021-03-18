table! {
    authors (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        headimageurl -> Nullable<Varchar>,
        simpleintro -> Nullable<Text>,
        detailintro -> Nullable<Json>,
    }
}

table! {
    poems (id) {
        id -> Int4,
        _id -> Nullable<Varchar>,
        title -> Varchar,
        dynasty -> Nullable<Varchar>,
        writer -> Nullable<Varchar>,
        poemtype -> Nullable<Array<Text>>,
        content -> Nullable<Text>,
        remark -> Nullable<Text>,
        translation -> Nullable<Text>,
        shangxi -> Nullable<Text>,
    }
}

table! {
    sentence (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        from -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    poems,
    sentence,
);
