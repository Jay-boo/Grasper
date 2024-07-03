# Reddit Wrapper 

Generate Combine Stream Fetching new post on multiple subreddit and sending real-time data to kafka broker


Test its integration with kafka using `docker-compose.yml`
1. Create `.env` file 
2. Run using docker compose :

```
cd reddit-wrapper
docker compose --env-file .env  up --build
```

