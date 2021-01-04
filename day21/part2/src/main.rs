use std::collections::HashSet;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

const NEW_LINE: &str = "\r\n";

#[derive(Clone, Hash, Eq, PartialEq, Debug, PartialOrd, Ord)]
struct Allergen(String);

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Ingredient(String);

struct FoodLabel {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>
}

impl FromStr for FoodLabel {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE_REGEX: Regex = Regex::new("^([a-z ]+) \\(contains ([a-z, ]+)\\)$").unwrap();
        }
        match LINE_REGEX.captures(line) {
            Some(line_match) => {
                let ingredients: HashSet<Ingredient> = line_match.get(1).unwrap().as_str().split(" ").map(|s| Ingredient(s.to_string())).collect();
                let allergens: HashSet<Allergen> = line_match.get(2).unwrap().as_str().split(", ").map(|s| Allergen(s.to_string())).collect();
                Ok(FoodLabel { ingredients, allergens })
            },
            None => Err(format!("Line did not match regex: {}", line))
        }
    }
}

struct AllergenMap {
    ingredients_with_allergens: HashMap<Allergen, Ingredient>,
    ingredients_without_allergens: HashSet<Ingredient>
}

impl AllergenMap {
    fn from_labels(labels: &Vec<FoodLabel>) -> Result<Self, String> {
        let mut candidates: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
        let mut all_ingredients: HashSet<Ingredient> = HashSet::new();
        for label in labels {
            all_ingredients.extend(label.ingredients.clone());
            for allergen in label.allergens.iter() {
                let new_candidates = match candidates.remove(&allergen) {
                    Some(existing) => existing.intersection(&label.ingredients).cloned().collect(),
                    None => label.ingredients.clone()
                };
                candidates.insert(allergen.clone(), new_candidates);
            }
        }
        let mut matches: HashMap<Allergen, Ingredient> = HashMap::new();
        let mut remaining_ingredients: HashSet<Ingredient> = all_ingredients;
        while !candidates.is_empty() {
            let matched_allergen: Allergen = match candidates.iter().filter(|(_k,v)| v.len() == 1).nth(0) {
                Some((k,_v)) => k.clone(),
                None => return Err("No allergens have an ingredient match.".to_string())
            };
            let matched_ingredient: Ingredient = candidates.remove(&matched_allergen).unwrap().into_iter().nth(0).unwrap();
            remaining_ingredients.remove(&matched_ingredient);
            for other_candidate_sets in candidates.values_mut() {
                other_candidate_sets.remove(&matched_ingredient);
            }
            matches.insert(matched_allergen, matched_ingredient);
        }
        Ok(AllergenMap {
            ingredients_with_allergens: matches,
            ingredients_without_allergens: remaining_ingredients
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let labels: Vec<FoodLabel> = text.split(NEW_LINE).map(|s| s.parse()
            .expect(&format!("Error parsing FoodLabel: {}", s))).collect();
        let map = AllergenMap::from_labels(&labels).expect("Error matching allergens");
        for (allergen, ingredient) in map.ingredients_with_allergens.iter() {
            println!("{:?} contains {:?}", ingredient, allergen);
        }
        println!("");
        println!("Ingredients without allergens: {:?}", map.ingredients_without_allergens);
        println!("");
        let count: usize = map.ingredients_without_allergens.iter().map(|i| labels.iter().filter(|l| l.ingredients.contains(i)).count()).sum();
        println!("Number of times these appear on labels: {}", count);
        println!("");
        let mut dangerous_pairs: Vec<(&Allergen,&Ingredient)> = map.ingredients_with_allergens.iter().collect();
        dangerous_pairs.sort_by_key(|(a,_i)| *a);
        let dangerous_ingredients: Vec<String> = dangerous_pairs.into_iter().map(|(_a,i)| i.0.to_string()).collect();
        println!("Dangerous ingredients: {}", dangerous_ingredients.join(","));
    } else {
        println!("Please provide 1 argument: Filename");
    }
}