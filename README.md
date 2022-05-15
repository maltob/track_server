# Track Server
A web server that provides a kindle image based on your location via Overland for use with Kindle Online Screensaver, Android app coming shortly.

## How to use

1. Setup your kindle PW/PW2 for [Online Screensaver](https://www.mobileread.com/forums/showthread.php?t=236104)
2. Build the app with "cargo build -r"
3. Create a config folder then copying the example folder and renaming it something secret. You'll have something like config/fi908gfhg with the files in it
4. Edit the config.toml, locations.csv and add any fonts you want to reference
5. Point the online screensaver to the URL the server will run on, I recommend a reverse proxy. Example http://something.tld/locations/fi908gfhg/kindle/Secret
6. Install [Overland App](https://github.com/aaronpk/Overland-iOS) and point it to the URL. Ex: http://something.tld/locations/fi908gfhg 
7. Test out the screensaver update on your kindle

## Example photo
![Employee data](/repository/images/ExampleImage.png?raw=true "Employee Data title")