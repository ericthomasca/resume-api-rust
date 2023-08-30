use chrono::NaiveDate;

#[derive(Debug, sqlx::FromRow)]
pub struct Education {
    pub id: i32,
    pub degree: String,
    pub institution: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub achievements: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SkillCategory {
    pub id: i32,
    pub category_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Skill {
    pub id: i32,
    pub category_id: i32,
    pub skill_name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct WorkExperience {
    pub id: i32,
    pub position: String,
    pub employer: String,
    pub location: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub responsibilities: String,
}
