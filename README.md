# Rubik API 

Rubik API is a simple REST API that allows you to obtain information about all kinds of Rubik's cubes, such as
the 2x2, 3x3, 4x4, 5x5, 6x6, 7x7, Pyraminx, Megaminx, Skewb, Square-1, and more.

The information that can be pulled from this API includes: 
- the name of the cube 
- the type of the cube 
- the number of pieces 
- the number of sides 
- the number of stickers
- the year the cube was invented 
- the world record for this cube.

*Try it out here!* https://rubik-api.onrender.com/cubes

**DISCLAIMER:** due to the free plan of the page where this API is hosted, if it doesn't receive any requests for a
while, it will enter on sleep mode.
This could cause the first request to take a little longer than usual, so please be patient :).

## Technologies 📦

- Rust
- Rocket
- MongoDB
- Postman
- [Render](https://render.com/) hosting service

## Functionalities ⚙️

The common users of this API will be only able to perform GET requests to obtain information about the cubes, while
the admin users will be able to perform all CRUD operations.

The endpoints are the following:

- `/cubes` - GET: returns all the cubes in the database
- `/cube?id={id}` - GET: returns the cube with the given id
- `/add_cube` - POST: adds a new cube to the database given the body of the new cube
- `/update_cube?id={id}` - PUT: updates the cube with the given id given the body of the new cube
- `/delete_cube?id={id}` - DELETE: deletes the cube with the given id

Check out the Postman documentation [here](https://documenter.getpostman.com/view/30891886/2sA2xfYZ2X)!


