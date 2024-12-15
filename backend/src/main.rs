mod db;
use db::create_pool;
use db::models::User;
use db::schema::user;
use diesel_async::RunQueryDsl;
use eyre::Result;
use uuid::Uuid;

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "User {{ id: {}, name: {}, email: {} }}",
            self.id, self.name, self.email
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let pool = create_pool()?;

    let mut conn = pool.get().await?;
    let mut write_conn = pool.get().await?;

    let user = User {
        id: Uuid::new_v4(),
        name: "John Doe".to_string(),
        email: "j.doe@example.com".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    diesel::insert_into(user::table)
        .values(&user)
        .execute(&mut write_conn)
        .await?;

    let users = user::table.load::<User>(&mut conn).await.unwrap();
    println!("{:?}", users);

    Ok(())
}
