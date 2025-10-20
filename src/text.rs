use rand::Rng;

pub fn get_random_word() -> String
{
    let random_word = vec!
    [
        "cat",
        "house",
        "dog",
        "cheese",
        "videogame",
        "anime",
        "people",
        "bird",
        "galaxy",
        "peace",
        "car",
        "absolute",
        "romeo",
        "bob",
        "charger",
        "ferrari",
        "chair",
        "table",
        "star",
        "sun"
    ];

    let mut rng = rand::rng();
    let index = rng.random_range(0..random_word.len());
    random_word[index].to_string()

}