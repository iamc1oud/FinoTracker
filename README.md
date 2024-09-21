# FinoTracker
Rust powered Fino Tracker

# Setting up Database

**Pull the MongoDB docker image**

`docker pull mongodb/mongodb-community-server:latest`

**Run the image as a container**

`docker run --name mongodb -p 27017:27017 -d mongodb/mongodb-community-server:latest`

The -p 27017:27017 in this command maps the container port to the host port. This allows you to connect to MongoDB with a localhost:27017 connection string.