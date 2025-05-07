// use sqlx::MySqlPool;
//
// pub async fn get_country_by_code(pool: &MySqlPool, code: &str) -> sqlx::Result<Option<Country>> {
//     let country = sqlx::query_as!(
//         Country,
//         "SELECT * FROM countries WHERE code = ?",
//         code
//     )
//         .fetch_optional(pool)
//         .await?;
//
//     Ok(country)
// }
//
// pub async fn insert_country(pool: &MySqlPool, country: &Country) -> sqlx::Result<()> {
//     sqlx::query!(
//         "INSERT INTO countries (name, code, capital, region, subregion, population, area, currency, language, flag_url)
//          VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
//         country.name,
//         country.code,
//         country.capital,
//         country.region,
//         country.subregion,
//         country.population,
//         country.area,
//         country.currency,
//         country.language,
//         country.flag_url,
//     )
//         .execute(pool)
//         .await?;
//
//     Ok(())
// }
