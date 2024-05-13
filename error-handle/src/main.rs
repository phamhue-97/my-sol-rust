use std::collections::HashMap;
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
/// The book type provided by an external API.
#[derive(Debug)]
struct APIBook {
    title: String,
    description: Option<String>,
}
/// The book type you need in the rest of your program.
#[derive(Debug)]
struct Book {
    title: String,
    description: String,
}
fn first_element<T>(array: &[T]) -> Option<&T> {
    if array.len() > 0 {
        Option::Some(&array[0])
    } else {
        Option::None
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Ingredient {
    Avocado,
    Bacon,
    Egg,
}

#[derive(Debug, Eq, PartialEq)]
struct BaconAndEggs;

#[derive(Debug, Eq, PartialEq)]
enum PreparationError {
    NotEnoughOfIngredient {
        ingredient: Ingredient,
        required: u32,
        available: u32,
    },
    // Other possible errors that could occur would go here.
}

/// Helper function to check if stock contains enough of an ingredient.
fn ensure_available(
    stock: &HashMap<Ingredient, u32>,
    ingredient: Ingredient,
    required: u32,
) -> Result<(), PreparationError> {
    let available = stock.get(&ingredient).copied().unwrap_or_default();

    if available >= required {
        Ok(())
    } else {
        Err(PreparationError::NotEnoughOfIngredient {
            ingredient,
            required,
            available,
        })
    }
}

/// Attempts to prepare bacon and eggs with ingredients that are in stock. The
/// stock is updated depending on the result.
fn prepare_bacon_and_eggs(
    stock: &mut HashMap<Ingredient, u32>,
) -> Result<BaconAndEggs, PreparationError> {
    const REQUIRED: [(Ingredient, u32); 2] = [(Ingredient::Bacon, 1), (Ingredient::Egg, 2)];

    // Ensure everything is available.
    for (ingredient, required) in REQUIRED {
        // Notice the use of the error propagation operator here. You return from
        // `prepare_bacon_and_eggs` early if `ensure_available` returns an
        // error.
        ensure_available(stock, ingredient, required)?;
    }

    // Update the stock.
    for (ingredient, required) in REQUIRED {
        // Since you verified that the hashmap contains all ingredients above you
        // can unwrap here.
        *stock.get_mut(&ingredient).unwrap() -= required;
    }

    Ok(BaconAndEggs)
}

fn main() {
    let a = [1, 2, 3];
    let first_from_a = first_element(&a);
    println!("{first_from_a:?}");

    let b: [i32; 0] = [];
    let first_from_b = first_element(&b);
    println!("{first_from_b:?}");
    println!();

    // The book objects you "received" from an API.
    let api_books: Vec<APIBook> = vec![
        APIBook {
            title: "Samson and Rik".to_string(),
            description: Option::Some("Samson and Rik go on many adventures.".to_string()),
        },
        APIBook {
            title: "De Kameleon".to_string(),
            description: Option::None,
        },
    ];

    println!("api_books: {api_books:#?}");
    println!();

    // The book objects you would like to use throughout the rest of your program.
    let books: Vec<Book> = api_books
        .into_iter()
        .filter_map(|api_book| {
            // Deconstruct the APIBook into its parts.
            let APIBook { title, description } = api_book;

            // Return None if description is None, otherwise take the String out of the `Option<String>`.
            let description = match description {
                Option::Some(description) => description,
                Option::None => return None,
            };

            // Create Book from the parts.
            Some(Book { title, description })
        })
        .collect::<Vec<_>>();

    println!("books: {books:#?}");
    println!();

    // Create a map from ingredient to a count that represents availability.
    let mut stock: HashMap<Ingredient, u32> = [
        (Ingredient::Avocado, 3),
        (Ingredient::Bacon, 2),
        (Ingredient::Egg, 1),
    ]
    .into_iter()
    .collect();

    // You should expect an error because bacon and eggs requires 2 eggs but you only have 1 in stock.
    assert_eq!(
        prepare_bacon_and_eggs(&mut stock),
        Err(PreparationError::NotEnoughOfIngredient {
            ingredient: Ingredient::Egg,
            required: 2,
            available: 1,
        })
    );

    // Add half a dozen of eggs to the stock.
    *stock.get_mut(&Ingredient::Egg).unwrap() += 6;

    // Verify that you now get bacon and eggs.
    assert_eq!(prepare_bacon_and_eggs(&mut stock), Ok(BaconAndEggs));

    // Make sure that the stock has been updated.
    assert_eq!(
        stock,
        [
            (Ingredient::Avocado, 3),
            (Ingredient::Bacon, 1),
            (Ingredient::Egg, 5),
        ]
        .into_iter()
        .collect::<HashMap<_, _>>()
    );

    // match std::panic::catch_unwind(|| {
    //     panic!("Stop!");
    // }) {
    //     Ok(_) => println!("No panics occurred."),
    //     Err(_) => println!("The code panicked."),
    // }

    struct Boom;

    impl Drop for Boom {
        fn drop(&mut self) {
            panic!("Boom!");
        }
    }
    let _boom = Boom;
    panic!("Stop!");
}
