use tfiala_mongodb_migrator::migration::Migration;

mod v001_add_accounts;

pub fn get_migrations() -> Vec<Box<dyn Migration>> {
    vec![Box::new(v001_add_accounts::Migration {})]
}
