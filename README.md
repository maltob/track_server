# Track Server
A web server that provides a kindle image based on your location via Overland for use with Kindle Online Screensaver or other endpoint such as an android app.

## How to use

### Manual
1. Setup your kindle PW/PW2 for [Online Screensaver](https://www.mobileread.com/forums/showthread.php?t=236104)
2. Build the app with "cargo build -r"
3. Create a config folder then copying the example folder and renaming it something secret. You'll have something like config/fi908gfhg with the files in it
4. Edit the config.toml, locations.csv and add any fonts you want to reference
5. Point the online screensaver to the URL the server will run on, I recommend a reverse proxy. Example http://something.tld/locations/fi908gfhg/kindle/Secret
6. Install [Overland App](https://github.com/aaronpk/Overland-iOS) and point it to the URL. Ex: http://something.tld/locations/fi908gfhg 
7. Test out the screensaver update on your kindle

### Docker
1. Install docker and docker-compose
2. Use docker-compose.example.yml as a base
3. Create a config folder then copying the example folder and renaming it something secret. You'll have something like config/fi908gfhg with the files in it
4. Edit the config.toml, locations.csv and add any fonts you want to reference
5. Setup the docker-compose either to directly serve (not recommended) or behind a proxy such as nginx
6. Install [Overland App](https://github.com/aaronpk/Overland-iOS) and point it to the URL. Ex: http://something.tld/locations/fi908gfhg 
7. Use the JSON endpoint to pull in data for your app/webpage/display. Ex: http://something.tld/locations/fi908gfhg/json/SecretPhrase


### Calendar endpoint
1. Enable the calendar upload point by setting a calendar_password in the config.toml
2. Create a csv file with start,end,text,media_url fields in your preferred scripting language
3. Post the .csv to http://something.tld/locations/fi908gfhg/calendar/SecretCalendarPhrase, replacing the key and SecretCalendarPhrase with the appropriate ones
4. Currently running calendar events will now override your location when you get the JSON status at Ex: http://something.tld/locations/fi908gfhg/json/SecretPhrase, if you want to get the location text while ignoring the calendar events add a ?location_only=false to the URL

## Example photo
![Employee data](/images/ExampleImage.png?raw=true "Example of the image generated")

