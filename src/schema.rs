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
