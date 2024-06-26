use sqlx::{Execute, PgPool, Postgres, QueryBuilder};

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://chat01:123456@localhost/chat01")
        .await
        .unwrap();
    let mut sql_builder: QueryBuilder<Postgres> = QueryBuilder::new("update chats set ");
    let mut separated = sql_builder.separated(", ");
    separated.push("name=122");

    separated.push("age=232");
    let query = sql_builder.build();
    println!("{:?}", query.sql());
    let _updated_rows = query.execute(&pool).await.unwrap().rows_affected();
}
