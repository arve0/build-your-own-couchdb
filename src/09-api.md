# Defining an API
Before we add a communication layer, lets define the API such that it's clear how the communication protocol needs to be.

I envision an API something like this:

```rs
let db = Database::new();
let id = UUID::new_v4();
let documents = db.get(id);
// no documents yet
assert(documents, vec![])

let new_documents = db.get_since(id, counter)

```
