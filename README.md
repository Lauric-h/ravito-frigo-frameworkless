# Frigo API
Frigo is a REST API designed to manage a list of food items and their characteristics. It is part of a larger project called Ravito.   
Frigo is implemented in Rust without using any frameworks, serving as a learning and practice exercise in Rust programming.   

Later iterations of the project will involve using a framework and adding more features for deployment in real-world scenarios.

## Features
Food Management: Frigo allows users to perform CRUD operations (Create, Read, Update, Delete) on food items. Users can add new food items, retrieve details of existing food items, update food item information, and delete food items.


## Install
Clone the Repository: Clone the Frigo repository to your local machine using Git.

```bash
git clone https://github.com/your-username/frigo.git
cd frigo
cargo run
```

**Access the API**: Frigo API will be accessible at http://localhost:8081.

## Usage 
- GET /foods: Retrieve a list of all food items.
- POST /foods: Add a new food item. 
- GET /foods/{id}: Retrieve details of a specific food item by ID.
- PUT /foods/{id}: Update information of a specific food item by ID.
- DELETE /foods/{id}: Delete a specific food item by ID.

An OpenAPI doc is available at `/docs/openapi.yaml`. 



