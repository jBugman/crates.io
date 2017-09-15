table! {
    /// Representation of the `api_tokens` table.
    ///
    /// (Automatically generated by Diesel.)
    api_tokens (id) {
        /// The `id` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `token` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Varchar,
        /// The `name` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `created_at` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `last_used_at` column of the `api_tokens` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        last_used_at -> Nullable<Timestamp>,
    }
}

table! {
    /// Representation of the `badges` table.
    ///
    /// (Automatically generated by Diesel.)
    badges (crate_id, badge_type) {
        /// The `crate_id` column of the `badges` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `badge_type` column of the `badges` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        badge_type -> Varchar,
        /// The `attributes` column of the `badges` table.
        ///
        /// Its SQL type is `Jsonb`.
        ///
        /// (Automatically generated by Diesel.)
        attributes -> Jsonb,
    }
}

table! {
    /// Representation of the `categories` table.
    ///
    /// (Automatically generated by Diesel.)
    categories (id) {
        /// The `id` column of the `categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `category` column of the `categories` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        category -> Varchar,
        /// The `slug` column of the `categories` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        slug -> Varchar,
        /// The `description` column of the `categories` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Varchar,
        /// The `crates_cnt` column of the `categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crates_cnt -> Int4,
        /// The `created_at` column of the `categories` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    /// Representation of the `crate_downloads` table.
    ///
    /// (Automatically generated by Diesel.)
    crate_downloads (crate_id, date) {
        /// The `crate_id` column of the `crate_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `downloads` column of the `crate_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        downloads -> Int4,
        /// The `date` column of the `crate_downloads` table.
        ///
        /// Its SQL type is `Date`.
        ///
        /// (Automatically generated by Diesel.)
        date -> Date,
    }
}

table! {
    /// Representation of the `crate_owner_invitations` table.
    ///
    /// (Automatically generated by Diesel.)
    crate_owner_invitations (invited_user_id, crate_id) {
        /// The `invited_user_id` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        invited_user_id -> Int4,
        /// The `invited_by_user_id` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        invited_by_user_id -> Int4,
        /// The `crate_id` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `created_at` column of the `crate_owner_invitations` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    /// Representation of the `crate_owners` table.
    ///
    /// (Automatically generated by Diesel.)
    crate_owners (crate_id, owner_id, owner_kind) {
        /// The `crate_id` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `owner_id` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        owner_id -> Int4,
        /// The `created_at` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `created_by` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        created_by -> Nullable<Int4>,
        /// The `deleted` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        deleted -> Bool,
        /// The `updated_at` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `owner_kind` column of the `crate_owners` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        owner_kind -> Int4,
    }
}

table! {
    /// Representation of the `crates` table.
    ///
    /// (Automatically generated by Diesel.)
    crates (id) {
        /// The `id` column of the `crates` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `name` column of the `crates` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
        /// The `updated_at` column of the `crates` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `created_at` column of the `crates` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `downloads` column of the `crates` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        downloads -> Int4,
        /// The `description` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Nullable<Varchar>,
        /// The `homepage` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        homepage -> Nullable<Varchar>,
        /// The `documentation` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        documentation -> Nullable<Varchar>,
        /// The `readme` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        readme -> Nullable<Varchar>,
        /// The `textsearchable_index_col` column of the `crates` table.
        ///
        /// Its SQL type is `TsVector`.
        ///
        /// (Automatically generated by Diesel.)
        textsearchable_index_col -> ::diesel_full_text_search::TsVector,
        /// The `license` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        license -> Nullable<Varchar>,
        /// The `repository` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        repository -> Nullable<Varchar>,
        /// The `max_upload_size` column of the `crates` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        max_upload_size -> Nullable<Int4>,
    }
}

table! {
    /// Representation of the `crates_categories` table.
    ///
    /// (Automatically generated by Diesel.)
    crates_categories (crate_id, category_id) {
        /// The `crate_id` column of the `crates_categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `category_id` column of the `crates_categories` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        category_id -> Int4,
    }
}

table! {
    /// Representation of the `crates_keywords` table.
    ///
    /// (Automatically generated by Diesel.)
    crates_keywords (crate_id, keyword_id) {
        /// The `crate_id` column of the `crates_keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `keyword_id` column of the `crates_keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        keyword_id -> Int4,
    }
}

table! {
    /// Representation of the `dependencies` table.
    ///
    /// (Automatically generated by Diesel.)
    dependencies (id) {
        /// The `id` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `version_id` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `crate_id` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `req` column of the `dependencies` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        req -> Varchar,
        /// The `optional` column of the `dependencies` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        optional -> Bool,
        /// The `default_features` column of the `dependencies` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        default_features -> Bool,
        /// The `features` column of the `dependencies` table.
        ///
        /// Its SQL type is `Array<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        features -> Array<Text>,
        /// The `target` column of the `dependencies` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        target -> Nullable<Varchar>,
        /// The `kind` column of the `dependencies` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        kind -> Int4,
    }
}

table! {
    /// Representation of the `emails` table.
    ///
    /// (Automatically generated by Diesel.)
    emails (id) {
        /// The `id` column of the `emails` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `user_id` column of the `emails` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `email` column of the `emails` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Varchar,
        /// The `verified` column of the `emails` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        verified -> Bool,
    }
}

table! {
    /// Representation of the `follows` table.
    ///
    /// (Automatically generated by Diesel.)
    follows (user_id, crate_id) {
        /// The `user_id` column of the `follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Int4,
        /// The `crate_id` column of the `follows` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
    }
}

table! {
    /// Representation of the `keywords` table.
    ///
    /// (Automatically generated by Diesel.)
    keywords (id) {
        /// The `id` column of the `keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `keyword` column of the `keywords` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        keyword -> Varchar,
        /// The `crates_cnt` column of the `keywords` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crates_cnt -> Int4,
        /// The `created_at` column of the `keywords` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    /// Representation of the `metadata` table.
    ///
    /// (Automatically generated by Diesel.)
    metadata (total_downloads) {
        /// The `total_downloads` column of the `metadata` table.
        ///
        /// Its SQL type is `Int8`.
        ///
        /// (Automatically generated by Diesel.)
        total_downloads -> Int8,
    }
}

table! {
    /// Representation of the `readme_rendering` table.
    ///
    /// (Automatically generated by Diesel.)
    readme_rendering (version_id) {
        /// The `version_id` column of the `readme_rendering` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `rendered_at` column of the `readme_rendering` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        rendered_at -> Nullable<Timestamp>,
    }
}

table! {
    /// Representation of the `reserved_crate_names` table.
    ///
    /// (Automatically generated by Diesel.)
    reserved_crate_names (name) {
        /// The `name` column of the `reserved_crate_names` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
    }
}

table! {
    /// Representation of the `teams` table.
    ///
    /// (Automatically generated by Diesel.)
    teams (id) {
        /// The `id` column of the `teams` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `login` column of the `teams` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        login -> Varchar,
        /// The `github_id` column of the `teams` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        github_id -> Int4,
        /// The `name` column of the `teams` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Nullable<Varchar>,
        /// The `avatar` column of the `teams` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        avatar -> Nullable<Varchar>,
    }
}

table! {
    /// Representation of the `tokens` table.
    ///
    /// (Automatically generated by Diesel.)
    tokens (id) {
        /// The `id` column of the `tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `email_id` column of the `tokens` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        email_id -> Int4,
        /// The `token` column of the `tokens` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        token -> Varchar,
        /// The `created_at` column of the `tokens` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
    }
}

table! {
    /// Representation of the `users` table.
    ///
    /// (Automatically generated by Diesel.)
    users (id) {
        /// The `id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `email` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        email -> Nullable<Varchar>,
        /// The `gh_access_token` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        gh_access_token -> Varchar,
        /// The `gh_login` column of the `users` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        gh_login -> Varchar,
        /// The `name` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Nullable<Varchar>,
        /// The `gh_avatar` column of the `users` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        gh_avatar -> Nullable<Varchar>,
        /// The `gh_id` column of the `users` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        gh_id -> Int4,
    }
}

table! {
    /// Representation of the `version_authors` table.
    ///
    /// (Automatically generated by Diesel.)
    version_authors (id) {
        /// The `id` column of the `version_authors` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `version_id` column of the `version_authors` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `user_id` column of the `version_authors` table.
        ///
        /// Its SQL type is `Nullable<Int4>`.
        ///
        /// (Automatically generated by Diesel.)
        user_id -> Nullable<Int4>,
        /// The `name` column of the `version_authors` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Varchar,
    }
}

table! {
    /// Representation of the `version_downloads` table.
    ///
    /// (Automatically generated by Diesel.)
    version_downloads (id) {
        /// The `id` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `version_id` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        version_id -> Int4,
        /// The `downloads` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        downloads -> Int4,
        /// The `counted` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        counted -> Int4,
        /// The `date` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Date`.
        ///
        /// (Automatically generated by Diesel.)
        date -> Date,
        /// The `processed` column of the `version_downloads` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        processed -> Bool,
    }
}

table! {
    /// Representation of the `versions` table.
    ///
    /// (Automatically generated by Diesel.)
    versions (id) {
        /// The `id` column of the `versions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Int4,
        /// The `crate_id` column of the `versions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        crate_id -> Int4,
        /// The `num` column of the `versions` table.
        ///
        /// Its SQL type is `Varchar`.
        ///
        /// (Automatically generated by Diesel.)
        num -> Varchar,
        /// The `updated_at` column of the `versions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        updated_at -> Timestamp,
        /// The `created_at` column of the `versions` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        created_at -> Timestamp,
        /// The `downloads` column of the `versions` table.
        ///
        /// Its SQL type is `Int4`.
        ///
        /// (Automatically generated by Diesel.)
        downloads -> Int4,
        /// The `features` column of the `versions` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        features -> Nullable<Varchar>,
        /// The `yanked` column of the `versions` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        yanked -> Bool,
        /// The `license` column of the `versions` table.
        ///
        /// Its SQL type is `Nullable<Varchar>`.
        ///
        /// (Automatically generated by Diesel.)
        license -> Nullable<Varchar>,
    }
}

joinable!(follows -> users (user_id));
joinable!(version_authors -> users (user_id));
joinable!(crates_keywords -> keywords (keyword_id));
joinable!(crates_categories -> categories (category_id));
joinable!(api_tokens -> users (user_id));
joinable!(crate_downloads -> crates (crate_id));
joinable!(crate_owners -> crates (crate_id));
joinable!(crates_categories -> crates (crate_id));
joinable!(crates_keywords -> crates (crate_id));
joinable!(dependencies -> crates (crate_id));
joinable!(follows -> crates (crate_id));
joinable!(versions -> crates (crate_id));
joinable!(dependencies -> versions (version_id));
joinable!(version_authors -> versions (version_id));
joinable!(version_downloads -> versions (version_id));
joinable!(crate_owners -> teams (owner_id));
joinable!(crate_owners -> users (owner_id));
joinable!(readme_rendering -> versions (version_id));
joinable!(crate_owner_invitations -> crates (crate_id));
joinable!(tokens -> emails (email_id));
