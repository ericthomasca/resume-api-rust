use sqlx::postgres::PgPoolOptions;
use std::env;
use sqlx::Error; // Import sqlx::Error to handle Result

mod model;

use crate::model::Education;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Problem importing .env file"); // Fixed typo

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Problem obtaining connection pool");

    println!("{:?}", pool);

    if let Err(err) = print_database_data(&pool).await {
        eprintln!("Error printing database data: {}", err);
    }
    
    if let Err(err) = print_skills_with_categories(&pool).await {
        eprintln!("Error printing skills with categories: {}", err);
    }

    Ok(())
}

async fn print_database_data(pool: &sqlx::PgPool) -> Result<(), Error> {
    let educations: Vec<Education> = sqlx::query_as("SELECT * FROM education")
        .fetch_all(pool)
        .await?;

    // Fetch other data (SkillCategory, Skill, WorkExperience) here

    println!("Educations:");
    for education in educations {
        println!("{:?}", education);
    }

    Ok(())
}

async fn print_skills_with_categories(pool: &sqlx::PgPool) -> Result<(), Error> {
    let query = r#"
        SELECT s.id AS skill_id, s.skill_name, c.category_name
        FROM skills s
        INNER JOIN skill_categories c ON s.category_id = c.id
    "#;

    let skills_with_categories: Vec<(i32, String, String)> = sqlx::query_as(query)
        .fetch_all(pool)
        .await?;

    println!("Skills with Categories:");
    for (_skill_id, skill_name, category_name) in skills_with_categories {
        println!("Skill Name: {}, Category Name: {}", skill_name, category_name);
    }

    Ok(())
}
