# 🧩 Rubik API 

Rubik API is a simple REST API that allows you to obtain information about the most popular speedcubing Rubik's cubes, 
such as the 2x2, 3x3, 4x4, 5x5, 6x6, 7x7, Pyraminx, Megaminx, Skewb, and Square-1.

The information that can be pulled from this API includes: 
- the name of the cube 
- the type of the cube 
- the number of pieces 
- the number of sides 
- the number of stickers
- the year the cube was invented 
- the world record for this cube.

*Try it out here!* https://rubik-api-jcasben.koyeb.app/cubes

## 📦 Technologies 

- Rust
- Rocket
- MongoDB
- Docker
- Postman
- Hosted at [Koyeb](https://www.koyeb.com/)

## ⚙️ Functionalities 

The common users of this API will be only able to perform GET requests to obtain information about the cubes, while
the admin users will be able to perform all CRUD operations.

The endpoints are the following ones:

- `/cubes` - GET: returns all the cubes in the database
- `/cube_by_id?id={id}` - GET: returns the cube with the given id
- `/cube_by_name?name={name}` - GET: returns the cube that matches the given name
- `/cube_by_type?type_={type}` - GET: returns all the cubes that match the given type
- `/add_cube` - POST: adds a new cube to the database given the body of the new cube
- `/update_cube?id={id}` - PUT: updates the cube with the given id with the given body
- `/update_by_name?name={name}` - PUT: updates the cube with the given name with the given body
- `/delete_cube?id={id}` - DELETE: deletes the cube with the given id

Check out the Postman documentation [here](https://documenter.getpostman.com/view/30891886/2sA2xfYZ2X)!