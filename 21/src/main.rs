use std::fs::read_to_string;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let parsed_input = parse_input(&input_string);
    let mappings = get_ingredient_allergene_mappings(&parsed_input);
    let all_ingredients_in_mappings = BTreeSet::from_iter(mappings.iter().map(|(_, i)| i.iter()).flatten());
    let not_contained_ingredients = parsed_input.iter()
        .map(|(i, _)| i)
        .flatten()
        .filter(|e| !all_ingredients_in_mappings.contains(e)).collect::<Vec<&&str>>();
    println!("{}", not_contained_ingredients.len());
    let final_mappings = find_final_mappings(&mappings);
    println!("{}", final_mappings.iter().map(|(_, v)| *v).collect::<Vec<&str>>().join(","));
}

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input.split("\n")
        .filter(|e| !e.is_empty())
        .map(|l| {
            let mut parts_iter = l.split(" (contains ");
            let ingredients_list = parts_iter.next().unwrap().split(" ").collect();
            let allergens_list_string = parts_iter.next().unwrap();
            let allergens_list = allergens_list_string[..allergens_list_string.len()-1].split(", ").collect();
            (ingredients_list, allergens_list)
        })
        .collect()
}

fn get_ingredient_allergene_mappings<'a>(inputs: &Vec<(Vec<&'a str>, Vec<&'a str>)>) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
    let allergenes_set = BTreeSet::from_iter(inputs.iter().map(|(_, a)| a).flatten());
    let mut map = BTreeMap::new();
    for allergen in allergenes_set.iter() {
        let ingredient_lists: Vec<&Vec<&str>> = inputs.iter()
            .filter(|(_, a)| a.contains(allergen))
            .map(|(i, _)| i)
            .collect();
        for ingredient in ingredient_lists[0].iter() {
            if ingredient_lists.iter().all(|i| i.contains(ingredient)) {
                (*map.entry(**allergen).or_insert(BTreeSet::new())).insert(*ingredient);
            }
        }
    }
    map
}

fn find_final_mappings<'a>(mappings: &BTreeMap<&'a str, BTreeSet<&'a str>>) -> BTreeMap<&'a str, &'a str> {
    let mut final_map = BTreeMap::new();
    while final_map.len() < mappings.len() {
        let mut new_values: BTreeMap<&str, &str> = BTreeMap::new();
        for (k, vs) in mappings.iter()
            .filter(|(k, _)| !final_map.contains_key(*k)) {
            let not_used_ingredients = vs.iter()
                .filter(|i| !final_map.values().any(|e| e == *i))
                .collect::<Vec<&&str>>();
            if not_used_ingredients.len() == 1 {
                new_values.insert(*k, *not_used_ingredients[0]);
            }
        }
        final_map.append(&mut new_values);
    }
    final_map
}
