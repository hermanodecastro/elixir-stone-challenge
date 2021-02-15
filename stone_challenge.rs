use std::collections::HashMap;

struct Item {
	_name: String,
	quantity: i32,
	unity_price: i32
}

struct Email {
	address: String
}

// This function checks whether the division is not exact. True if it is not an exact division, and False if it is.
fn is_not_exact_division(list_total: &i32, amount_of_people: &i32) -> bool {
	list_total % amount_of_people != 0
}

// This function takes as arguments a vector of Item and a vector of Email and returns a HashMap on success or Error on failure. 
fn calculates(items: &Vec<Item>, emails: &Vec<Email>) -> Result<HashMap<String, i32>, String> {
	// Checks whether the item list or the email list is empty
	if emails.len() == 0 || items.len() == 0 {
		return Err(String::from("List of emails or items is empty!"));
	}

	let mut list_total = 0;
	let amount_of_people = emails.len() as i32;

	// Calculates the total value of the list
	for item in items {
		list_total += item.quantity * item.unity_price;
	} 

	// Individual value as integer.
	let personal_value = list_total / amount_of_people;

	// Creates the hashmap
	let mut result: HashMap<String, i32> = HashMap::new();

	if is_not_exact_division(&list_total, &amount_of_people) {
		// Takes the amount left over from multiplying the individual integer values ​​minus the total value in the list
		let mut _rest = list_total - personal_value * amount_of_people;

		for i in 0..emails.len() {
			// Distributes the remaining value among people until the value reaches zero.
			if _rest >= 1 {
				result.insert(emails[i].address.to_string(), personal_value + 1);
			} else {
				result.insert(emails[i].address.to_string(), personal_value);
			}
			// Decreases by one unit in each distribution
			_rest -= 1;
		}

	} else {
		// Here the value is shared equally
		for i in 0..emails.len() {
			result.insert(emails[i].address.to_string(), personal_value);
		}
	}

	Ok(result)
}

fn main() {
	// Creating list of items
	let items = vec![
		Item {
			_name: String::from("Item A"),
			quantity: 1,
			unity_price: 18
		},
		Item {
			_name: String::from("Item B"),
			quantity: 1,
			unity_price: 18
		},
		Item {
			_name: String::from("Item C"),
			quantity: 1,
			unity_price: 18
		},
		Item {
			_name: String::from("Item D"),
			quantity: 1,
			unity_price: 18
		},
		Item {
			_name: String::from("Item E"),
			quantity: 1,
			unity_price: 18
		},
		Item {
			_name: String::from("Item F"),
			quantity: 1,
			unity_price: 17
		},
	];

	// Creating list of emails
	let emails = vec![
		Email {
			address: String::from("hermano@gmail.com")
		},
		Email {
			address: String::from("brena@hotmail.com")
		},
		Email {
			address: String::from("lucas@gmail.com")
		},
		Email {
			address: String::from("ariele@gmail.com")
		},
		Email {
			address: String::from("fernando@gmail.com")
		},
		Email {
			address: String::from("lara@gmail.com")
		},
	];

	// Receives the hashmap
	let result = match calculates(&items, &emails) {
		Ok(result) => result,
		// In case of error, empty item list or empty email list, exits the program.
		Err(e) => panic!(e)
	};

	// Shows the result
	for (email, personal_value) in &result {
	    println!("{}: {}", email, personal_value);
	}

	// To run all tests, uncomment the function below.
	// test_all();
}

// As this is a single file and not a project made with Cargo, it is not possible to run the command "cargo test". 
// So the tests here are done manually.

#[allow(dead_code)]
fn test_all() {
	test_is_not_exact_division_should_return_true();
	test_is_not_exact_division_should_return_false();
	test_calculates_must_divide_in_34_33_33();
	test_calculates_must_divide_in_25_25_25();
	println!("\nAll tests passed!");
}

#[allow(dead_code)]
fn test_is_not_exact_division_should_return_true() {
	assert_eq!(true, is_not_exact_division(&100, &3));
}

#[allow(dead_code)]
fn test_is_not_exact_division_should_return_false() {
	assert_eq!(false, is_not_exact_division(&100, &2));
} 

#[allow(dead_code)]
fn test_calculates_must_divide_in_34_33_33() {
	// List total is 100. 
	// The first email in the vector must pay 34 and the others 33.
	let items = vec![
		Item {
			_name: String::from("Item A"),
			quantity: 25,
			unity_price: 1
		},
		Item {
			_name: String::from("Item B"),
			quantity: 25,
			unity_price: 1
		},
		Item {
			_name: String::from("Item C"),
			quantity: 50,
			unity_price: 1
		},
	];

	let emails = vec![
		Email {
			address: String::from("hermano@gmail.com")
		},
		Email {
			address: String::from("brena@hotmail.com")
		},
		Email {
			address: String::from("lucas@gmail.com")
		},
	];

	let result = match calculates(&items, &emails) {
		Ok(result) => result,
		Err(e) => panic!(e)
	};

	assert_eq!(Some(&34), result.get("hermano@gmail.com"));
	assert_eq!(Some(&33), result.get("brena@hotmail.com"));
	assert_eq!(Some(&33), result.get("lucas@gmail.com"));
}

#[allow(dead_code)] 
fn test_calculates_must_divide_in_25_25_25() {
	// List total is 75.
	// All emails in the vector must pay 25.
	let items = vec![
		Item {
			_name: String::from("Item A"),
			quantity: 25,
			unity_price: 1
		},
		Item {
			_name: String::from("Item B"),
			quantity: 25,
			unity_price: 1
		},
		Item {
			_name: String::from("Item C"),
			quantity: 25,
			unity_price: 1
		},
	];

	let emails = vec![
		Email {
			address: String::from("hermano@gmail.com")
		},
		Email {
			address: String::from("brena@hotmail.com")
		},
		Email {
			address: String::from("lucas@gmail.com")
		},
	];

	let result = match calculates(&items, &emails) {
		Ok(result) => result,
		Err(e) => panic!(e)
	};

	assert_eq!(Some(&25), result.get("hermano@gmail.com"));
	assert_eq!(Some(&25), result.get("brena@hotmail.com"));
	assert_eq!(Some(&25), result.get("lucas@gmail.com"));
}


