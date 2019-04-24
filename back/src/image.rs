struct Image {
  name: String,
  svg: String,
  png: String
}

impl Image {
    fn create(image: Json<Image>) -> Json<Image> {
        hero
    }
]
    fn read() -> Json<Image> {
        Json(json!([
            "hero 1", 
            "hero 2"
        ]))
    }

    fn update(id: i32, image: Json<Hero>) -> Json<Image> {
        hero
    }

    fn delete(id: i32) -> image<Image> {
        Json(json!({"status": "ok"}))
    }
}