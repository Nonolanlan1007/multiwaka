# Multi Waka
This project allows you to use two instances of Wakatime at once. It's particularly useful for me to be able to participate to HackClub's challenges while still be able to use Wakatime.

## Installation instructions
Download the compose file
```shell
curl https://raw.githubusercontent.com/Nonolanlan1007/multiwaka/refs/heads/master/docker-compose.yml
```
Update the environnement variables in `docker-compose.yml` and then update your `.wakatime.cfg` :
```cfg
[settings]
api_url = http://127.0.0.1:1234
```
Then restart your IDE and enjoy :)