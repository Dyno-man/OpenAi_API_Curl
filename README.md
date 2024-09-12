This Repo is designed for making basic requests to the OpenAi_API using the curl library. Feel free to use it!

This library was designed on Mac OS so setting your API key is the same for Linux/Mac Os users.
  To set your API key in your project directry type the command: export OPENAI_API_KEY = "Whatever your Api key is"

For windows users you will have to edit the get_api_key method slightly using the dotenv::dotenv library plus the command dotenv().ok()
then in a seperate .env file set your api key to OPENAI_API_KEY ="Whatever your Api key is"

The main.rs file is an example of how to use the library, it is still in production so it doesn't have many functions besides communicating with the OpenAi Api.
