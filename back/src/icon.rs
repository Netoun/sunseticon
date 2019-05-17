use postgres::rows::Row;
use postgres::Connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
  pub name: String,
  pub png: String
}

impl Icon {
   pub fn create(icon: Icon, connection: &Connection) -> Icon{
      connection
         .execute(
         r#"INSERT INTO "SUBSCRIPTION" (name, png) VALUES ($1, $2)"#,
         &[&icon.name, &icon.png],
         )
         .unwrap();
      icon
   }

   pub fn get_all (connection: &Connection) -> Vec<Icon> {
      connection
      .query(r#"SELECT * from "ICON"#, &[])
      .unwrap()
      .into_iter()
      .map(|row| Icon {
        name: row.get(0),
        png: row.get(1),
      })
      .collect::<Vec<_>>()
   }
}