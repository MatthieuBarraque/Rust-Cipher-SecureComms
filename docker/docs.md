# Build the images
docker-compose build

# Start the server
docker-compose up server

# In separate terminals, start clients
docker-compose run client1
docker-compose run client2