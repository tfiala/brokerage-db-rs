use tfiala_mongodb_migrator::migration::Migration;

mod v001_add_accounts;
mod v002_add_security;

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![
        Box::new(v001_add_accounts::Migration001 {}),
        Box::new(v002_add_security::Migration002 {}),
    ]
}
