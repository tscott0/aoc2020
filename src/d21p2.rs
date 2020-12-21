use std::collections::HashMap;

use crate::utils::string_from_file;

pub fn run() {
    let input = string_from_file("src/21input");

    println!("{}", solve(input));
}

pub fn solve(input: String) -> String {
    let mut all_allergens: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    input.lines().for_each(|l| {
        let mut parts = l.split(" (contains ");
        let ingredients = parts
            .next()
            .expect("Ingredients exist")
            .trim()
            .to_string()
            .split(" ")
            .map(|i| i.to_string())
            .collect::<Vec<String>>();

        parts
            .next()
            .expect("Allergens exist")
            .split(",")
            .map(|a| a.replace(")", "").trim().to_owned())
            .for_each(|a| {
                let related_foods = all_allergens.entry(a.clone()).or_default();
                related_foods.push(ingredients.clone());
            });
    });

    println!("allergens {:?}", all_allergens);

    let mut resolved_allergens: HashMap<String, String> = HashMap::new();

    while all_allergens.len() > 0 {
        let mut allergen_to_remove: Option<(String, String)> = None;

        all_allergens.iter().for_each(|(a, _)| {
            match common_ingredients(a.clone(), &all_allergens) {
                Some(ingredients) => {
                    if ingredients.len() == 1 {
                        println!("allergen {} must be {}", a, ingredients[0]);

                        allergen_to_remove = Some((a.clone(), ingredients[0].clone()));
                        resolved_allergens.insert(a.clone(), ingredients[0].clone());
                    } else {
                        println!("allergen {} must be one of {:?}", a, ingredients);
                    }
                }
                None => {}
            }
        });

        match allergen_to_remove {
            Some((a, i)) => {
                all_allergens.remove(&a);

                all_allergens.iter_mut().for_each(|(_, foods)| {
                    foods.iter_mut().for_each(|f| {
                        let pos_op = f
                            .iter_mut()
                            .position(|ingredient| ingredient.to_string() == i);
                        match pos_op {
                            Some(pos) => {
                                f.remove(pos);
                            }
                            None => {}
                        };
                    })
                });
            }
            None => {}
        }
    }

    println!("resolved {:?}", resolved_allergens);

    let mut sorted = resolved_allergens.iter().collect::<Vec<_>>();

    sorted.sort_by(|(a, _), (b, _)| a.cmp(b));

    let sorted_ing = sorted
        .iter()
        .map(|(_, i)| i.clone().clone())
        .collect::<Vec<_>>();

    sorted_ing.join(",")
}

fn common_ingredients(
    allergen: String,
    all_allergens: &HashMap<String, Vec<Vec<String>>>,
) -> Option<Vec<String>> {
    match all_allergens.get(&allergen) {
        Some(ingredients) => match ingredients.iter().next() {
            Some(food) => {
                let found = food
                    .iter()
                    .filter_map(|i| {
                        if ingredients.iter().all(|f| f.contains(i)) {
                            Some(i.to_string())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<String>>();

                Some(found)
            }
            None => None,
        },
        None => None,
    }
}

#[test]
fn example_1() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)";

    assert_eq!(solve(input.to_string()), "mxmxvkd,sqjhc,fvjkl".to_string());
}
